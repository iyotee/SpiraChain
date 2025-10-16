use crate::{BlockStorage, NodeConfig, WorldState};
use spirachain_consensus::{ProofOfSpiral, SlotConsensus, Validator};
use spirachain_core::{Address, Amount, Block, Result, Transaction};
use spirachain_crypto::{KeyPair, PublicKey};
use spirachain_network::{LibP2PNetworkWithSync, NetworkEvent};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, warn};

pub struct ValidatorNode {
    config: NodeConfig,
    keypair: KeyPair,
    validator: Validator,
    mempool: Arc<RwLock<Vec<Transaction>>>,
    state: Arc<RwLock<WorldState>>,
    storage: Arc<BlockStorage>,
    consensus: ProofOfSpiral,
    slot_consensus: Arc<RwLock<SlotConsensus>>,
    network: Option<Arc<RwLock<LibP2PNetworkWithSync>>>,
    is_running: Arc<RwLock<bool>>,
    blocks_produced: u64,
    connected_peers: Arc<RwLock<usize>>,
    current_height: Arc<RwLock<u64>>,
}

impl ValidatorNode {
    pub fn new(config: NodeConfig, keypair: KeyPair) -> Result<Self> {
        let storage = BlockStorage::new(&config.data_dir)?;
        let address = keypair.to_address();

        let validator = Validator {
            address,
            pubkey: keypair.public_key().as_bytes().to_vec(),
            stake: Amount::new(10_000 * 10u128.pow(18)),
            locked_until: 0,
            rewards_earned: Amount::new(0),
            slashing_events: Vec::new(),
            blocks_proposed: 0,
            expected_blocks: 0,
            reputation_score: 1.0,
            last_block_height: 0,
        };

        let mut consensus = ProofOfSpiral::new(
            spirachain_core::MIN_SPIRAL_COMPLEXITY,
            spirachain_core::MAX_SPIRAL_JUMP,
        );

        // Enregistrer ce validator dans le consensus
        consensus.add_validator(validator.clone())?;

        // Initialiser SpiraPi AI engine
        let spirapi_path = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("crates/spirapi");

        if spirapi_path.exists() {
            info!("🤖 Initializing SpiraPi AI engine...");
            match spirapi_bridge::SpiraPiEngine::initialize(spirapi_path) {
                Ok(_) => info!("✅ SpiraPi AI engine initialized successfully"),
                Err(e) => warn!(
                    "⚠️ SpiraPi not available: {}. Using fallback embeddings.",
                    e
                ),
            }
        } else {
            warn!("⚠️ SpiraPi directory not found. AI semantic layer will use fallback mode.");
        }

        // Get initial blockchain height
        let initial_height = storage
            .get_latest_block()
            .ok()
            .flatten()
            .map(|b| b.header.block_height)
            .unwrap_or(0);

        // Initialize slot consensus
        let mut slot_consensus = SlotConsensus::new(&config.network);
        // Register ourselves as a validator
        slot_consensus.add_validator(address);

        info!("🎰 Slot consensus initialized");
        info!("   Network: {}", config.network);
        info!(
            "   Slot duration: {}s",
            if config.network == "mainnet" { 60 } else { 30 }
        );

        Ok(Self {
            config,
            keypair,
            validator,
            mempool: Arc::new(RwLock::new(Vec::new())),
            state: Arc::new(RwLock::new(WorldState::default())),
            storage: Arc::new(storage),
            consensus,
            slot_consensus: Arc::new(RwLock::new(slot_consensus)),
            network: None, // Initialized in start()
            is_running: Arc::new(RwLock::new(false)),
            blocks_produced: 0,
            connected_peers: Arc::new(RwLock::new(0)),
            current_height: Arc::new(RwLock::new(initial_height)),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting SpiraChain Validator Node");
        info!("   Address: {}", self.validator.address);
        info!(
            "   Stake: {} QBT",
            self.validator.stake.value() as f64 / 1e18
        );
        info!("   Data dir: {}", self.config.data_dir.display());

        // Initialize P2P network with block sync
        info!("🌐 Starting LibP2P network with block synchronization...");
        let port = self
            .config
            .network_addr
            .split(':')
            .next_back()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(30333);

        let current_height = *self.current_height.read().await;
        info!("📊 Current blockchain height: {}", current_height);

        match LibP2PNetworkWithSync::new_with_network(port, &self.config.network, current_height)
            .await
        {
            Ok(mut network) => {
                info!(
                    "✅ P2P network with sync created for {}",
                    self.config.network.to_uppercase()
                );

                // Set up block storage callback
                let storage_clone = Arc::clone(&self.storage);
                let state_clone = Arc::clone(&self.state);
                let height_clone = Arc::clone(&self.current_height);

                network.set_block_store_callback(move |block: Block| {
                    let height = block.header.block_height;
                    info!("💾 Storing synced block {}", height);

                    // Store the block
                    storage_clone.store_block(&block)?;

                    // Update height
                    let rt = tokio::runtime::Handle::current();
                    rt.block_on(async {
                        let mut h = height_clone.write().await;
                        *h = height;
                    });

                    // Update state with block transactions
                    rt.block_on(async {
                        let mut state = state_clone.write().await;
                        for tx in &block.transactions {
                            if let Err(e) = state.transfer(&tx.from, &tx.to, tx.amount) {
                                warn!("Failed to apply transaction in synced block: {}", e);
                            }
                        }

                        // Persist all balances after applying block
                        for (address, balance) in state.get_all_balances() {
                            if let Err(e) = storage_clone.set_balance(&address, balance) {
                                warn!("Failed to persist balance for {}: {}", address, e);
                            }
                        }
                    });

                    Ok(())
                });

                // Initialize listening with bootstrap
                if let Err(e) = network.initialize_with_bootstrap().await {
                    warn!(
                        "⚠️ P2P initialization failed: {}. Running without network.",
                        e
                    );
                } else {
                    #[allow(clippy::arc_with_non_send_sync)]
                    {
                        self.network = Some(Arc::new(RwLock::new(network)));
                    }
                    info!("📡 P2P network ready with block sync - will poll in validator loop");
                    info!("🔍 Validators will be auto-discovered from blocks they produce");
                }
            }
            Err(e) => {
                warn!(
                    "⚠️ P2P network failed to create: {}. Running without network.",
                    e
                );
            }
        }

        // Credit initial staking balance ONLY for testnet (no staking required)
        // Mainnet: No initial credit, fair launch - everyone starts at 0 and earns through rewards
        if self.config.network == "testnet" {
            let current_balance = self
                .storage
                .get_balance(&self.validator.address)
                .unwrap_or_default();
            if current_balance.is_zero() {
                let initial_stake = Amount::new(1000 * 1e18 as u128); // 1000 QBT for testnet only
                if let Err(e) = self
                    .storage
                    .set_balance(&self.validator.address, initial_stake)
                {
                    warn!("Failed to set initial stake: {}", e);
                } else {
                    info!(
                        "💰 [TESTNET] Initial staking balance credited: {} QBT",
                        initial_stake.value() as f64 / 1e18
                    );

                    // Verify the balance was actually stored
                    match self.storage.get_balance(&self.validator.address) {
                        Ok(stored_balance) => {
                            info!(
                                "✅ Verified stored balance: {} QBT",
                                stored_balance.value() as f64 / 1e18
                            );
                        }
                        Err(e) => {
                            warn!("❌ Failed to verify stored balance: {}", e);
                        }
                    }
                }
            } else {
                info!(
                    "💰 Existing balance found: {} QBT (skipping initial credit)",
                    current_balance.value() as f64 / 1e18
                );
            }
        } else {
            // Mainnet: No initial credit - fair launch like Bitcoin
            info!("🚀 [MAINNET] Fair launch mode: Starting with 0 QBT, earn through block rewards only");
        }

        // Load validator balance from storage into WorldState
        // This ensures the initial stake is not lost when producing the first block
        let stored_balance = self
            .storage
            .get_balance(&self.validator.address)
            .unwrap_or_default();
        if !stored_balance.is_zero() {
            let mut state = self.state.write().await;
            state.set_balance(self.validator.address, stored_balance);
            info!(
                "🔄 Loaded validator balance into WorldState: {} QBT",
                stored_balance.value() as f64 / 1e18
            );
        }

        // Start RPC server
        let rpc_port = 8545;
        info!("🌐 Starting RPC server on port {}...", rpc_port);

        let mempool_clone = Arc::clone(&self.mempool);
        let storage_clone = Arc::clone(&self.storage);
        let chain_height = Arc::new(RwLock::new(0u64));
        let chain_height_clone = Arc::clone(&chain_height);
        let connected_peers_clone = Arc::clone(&self.connected_peers);

        tokio::spawn(async move {
            let rpc_server = spirachain_rpc::RpcServer::new(
                mempool_clone,
                storage_clone,
                chain_height_clone,
                connected_peers_clone,
                true,
                rpc_port,
            );

            if let Err(e) = rpc_server.start().await {
                error!("RPC server error: {}", e);
            }
        });

        info!("✅ RPC server started on port {}", rpc_port);

        *self.is_running.write().await = true;

        let latest_block = self.storage.get_latest_block()?;
        if let Some(block) = latest_block {
            info!("   Latest block: {}", block.header.block_height);
            *chain_height.write().await = block.header.block_height;
            self.state
                .write()
                .await
                .set_height(block.header.block_height);
        } else {
            info!("   No blocks yet - will create genesis");
        }

        self.run_validator_loop().await?;

        Ok(())
    }

    async fn run_validator_loop(&mut self) -> Result<()> {
        // Block timer matches slot duration (30s testnet, 60s mainnet)
        let block_interval = if self.config.network == "mainnet" { 60 } else { 30 };
        let mut block_timer = interval(Duration::from_secs(block_interval));
        let mut stats_timer = interval(Duration::from_secs(30));
        let mut mempool_check = interval(Duration::from_secs(5));
        let mut network_tick = interval(Duration::from_millis(100));

        info!("⚡ Validator loop started (slot duration: {}s)", block_interval);
        if self.network.is_some() {
            info!("   P2P network enabled");
        }

        loop {
            tokio::select! {
                _ = block_timer.tick() => {
                    // Check if it's our turn to produce a block (slot-based consensus)
                    let slot_consensus = self.slot_consensus.read().await;
                    let is_our_turn = slot_consensus.is_slot_leader(&self.validator.address);
                    let current_slot = slot_consensus.get_current_slot();
                    let validator_count = slot_consensus.validator_count();

                    if is_our_turn {
                        info!("✅ Our turn to produce block (slot {}, validators: {})", current_slot, validator_count);
                        drop(slot_consensus);
                        if let Err(e) = self.produce_block().await {
                            error!("Failed to produce block: {}", e);
                        }
                    } else {
                        let leader = slot_consensus.get_current_leader();
                        debug!("⏳ Waiting for our slot (current leader: {:?}, slot {}, validators: {})", leader, current_slot, validator_count);
                    }
                }

                _ = stats_timer.tick() => {
                    self.print_stats().await;
                }

                _ = mempool_check.tick() => {
                    self.check_mempool().await;
                }

                _ = network_tick.tick() => {
                    // Poll P2P events and handle network messages
                    if let Some(ref network) = self.network {
                        let event = {
                            let mut net = network.write().await;
                            let evt = net.poll_events().await;

                            // Update local height in sync manager
                            let current_height = *self.current_height.read().await;
                            net.set_local_height(current_height);

                            // Update connected peers count
                            let peer_count = net.peer_count();
                            *self.connected_peers.write().await = peer_count;

                            evt
                        };

                        // Handle event outside of the lock
                        if let Some(event) = event {
                            self.handle_network_event(event).await;
                        }
                    }
                }
            }

            if !*self.is_running.read().await {
                info!("Validator stopped");
                break;
            }
        }

        Ok(())
    }

    async fn produce_block(&mut self) -> Result<()> {
        info!("🏗️  Producing new block...");

        let mempool_guard = self.mempool.read().await;
        let pending_txs = mempool_guard.iter().take(1000).cloned().collect::<Vec<_>>();
        drop(mempool_guard);

        // Get latest block from storage (not state height!)
        let previous_block = self.storage.get_latest_block()?;

        let current_height = if let Some(ref prev) = previous_block {
            prev.header.block_height
        } else {
            0
        };

        info!("   Height: {} → {}", current_height, current_height + 1);
        info!("   Transactions: {}", pending_txs.len());

        let block = if let Some(prev_block) = previous_block {
            // Generate normal block
            self.consensus.generate_block_candidate(
                &self.validator,
                &self.keypair,
                pending_txs.clone(),
                &prev_block,
            )?
        } else {
            // Only create genesis if no blocks exist
            info!("   Creating genesis block");
            let config = spirachain_core::GenesisConfig::default();

            spirachain_core::create_genesis_block(&config)
        };

        // Only validate non-genesis blocks
        if current_height > 0 {
            if let Some(prev) = self.storage.get_latest_block()? {
                self.consensus.validate_block(&block, &prev)?;
            }
        }

        self.storage.store_block(&block)?;

        {
            let mut state = self.state.write().await;

            // Process transactions
            for tx in &block.transactions {
                if let Err(e) = state.transfer(&tx.from, &tx.to, tx.amount) {
                    warn!("Failed to transfer in block: {}", e);
                } else {
                    state.increment_nonce(&tx.from);
                }
            }

            // Credit block reward to validator
            let block_reward = Amount::new(spirachain_core::INITIAL_BLOCK_REWARD);
            state.credit_balance(&self.validator.address, block_reward);

            let new_balance = state.get_balance(&self.validator.address);
            info!(
                "💰 Crediting {} QBT to validator. New balance: {} QBT",
                block_reward.value() as f64 / 1e18,
                new_balance.value() as f64 / 1e18
            );

            // Persist validator balance to storage
            if let Err(e) = self
                .storage
                .set_balance(&self.validator.address, new_balance)
            {
                warn!("Failed to persist validator balance: {}", e);
            } else {
                info!("✅ Balance persisted to storage");
            }

            // Sync all balances from WorldState to BlockStorage
            for (address, balance) in state.get_all_balances() {
                if let Err(e) = self.storage.set_balance(&address, balance) {
                    warn!("Failed to sync balance for {}: {}", address, e);
                }
            }

            state.set_height(block.header.block_height);
        }

        let mut mempool_guard = self.mempool.write().await;
        mempool_guard.retain(|tx| !pending_txs.iter().any(|ptx| ptx.tx_hash == tx.tx_hash));
        drop(mempool_guard);

        self.blocks_produced += 1;
        self.validator.blocks_proposed += 1;
        self.validator.last_block_height = block.header.block_height;

        // Update current height
        *self.current_height.write().await = block.header.block_height;

        info!(
            "✅ Block {} produced successfully!",
            block.header.block_height
        );
        info!("   Hash: {}", block.hash());
        info!("   Transactions: {}", block.header.tx_count);

        // Broadcast block to P2P network
        if let Some(ref network) = self.network {
            let mut net = network.write().await;
            if let Err(e) = net.broadcast_block(&block).await {
                warn!("Failed to broadcast block: {}", e);
            } else {
                debug!(
                    "📡 Block {} broadcasted to {} peers",
                    block.header.block_height,
                    net.peer_count()
                );
            }
        }

        Ok(())
    }

    pub async fn submit_transaction(&mut self, tx: Transaction) -> Result<()> {
        info!(
            "📥 Received transaction: {} → {} ({} QBT)",
            tx.from.to_string()[..16].to_string(),
            tx.to.to_string()[..16].to_string(),
            tx.amount.value() as f64 / 1e18
        );

        tx.validate()?;

        let state = self.state.read().await;
        let balance = state.get_balance(&tx.from);
        drop(state);

        let required = Amount::new(tx.amount.value() + tx.fee.value());
        if balance < required {
            return Err(spirachain_core::SpiraChainError::InsufficientBalance);
        }

        let mut mempool_guard = self.mempool.write().await;
        mempool_guard.push(tx);
        drop(mempool_guard);

        Ok(())
    }

    async fn handle_network_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::PeerConnected(peer) => {
                info!("🤝 Peer connected: {}", peer);

                // For now, we'll discover validators by monitoring blocks they produce
                // Each block contains the validator's address in the signature
                // When we receive a block, we'll add that validator to our slot consensus
            }
            NetworkEvent::PeerDisconnected(peer) => {
                info!("👋 Peer disconnected: {}", peer);
            }
            NetworkEvent::PeerHeight { peer, height } => {
                debug!("📊 Peer {} has height: {}", peer, height);
                let current_height = *self.current_height.read().await;
                if height > current_height {
                    info!(
                        "🔄 Peer {} ahead by {} blocks, will sync...",
                        peer,
                        height - current_height
                    );
                }
            }
            NetworkEvent::NewBlock(block) => {
                let height = block.header.block_height;
                let current_height = *self.current_height.read().await;

                info!(
                    "📦 Received new block {} from network (current: {})",
                    height, current_height
                );

                // AUTO-DISCOVERY: Extract validator address from block and add to slot consensus
                debug!("🔍 Block validator_pubkey length: {}", block.header.validator_pubkey.len());
                if !block.header.validator_pubkey.is_empty() {
                    match PublicKey::from_bytes(&block.header.validator_pubkey) {
                        Ok(pubkey) => {
                            let validator_address = pubkey.to_address();
                            debug!("🔍 Extracted validator address: {}", validator_address);

                            // Add to slot consensus if not already registered
                            let mut slot_consensus = self.slot_consensus.write().await;
                            let before_count = slot_consensus.validator_count();
                            slot_consensus.add_validator(validator_address);
                            let after_count = slot_consensus.validator_count();

                            if after_count > before_count {
                                warn!(
                                    "📝 Discovered new validator: {} (total: {})",
                                    validator_address, after_count
                                );
                            } else {
                                debug!("Validator already known: {}", validator_address);
                            }
                            drop(slot_consensus);
                        }
                        Err(e) => {
                            warn!("Failed to extract validator address from block: {}", e);
                        }
                    }
                } else {
                    warn!("⚠️  Block {} has empty validator_pubkey!", height);
                }

                // Skip if we already have this block
                if height < current_height {
                    debug!(
                        "⊘ Skipping block {} - we are ahead (current: {})",
                        height, current_height
                    );
                    return;
                }

                // Basic validation
                if let Err(e) = block.validate() {
                    warn!("❌ Invalid block {} from network: {}", height, e);
                    return;
                }

                // FORK DETECTION: Check if this block connects to our chain
                let is_fork = if height > 0 {
                    if let Ok(Some(our_block)) = self.storage.get_block_by_height(height - 1) {
                        // Check if prev_hash matches
                        block.header.previous_block_hash != our_block.hash()
                    } else {
                        // We don't have the previous block, assume not a fork yet
                        false
                    }
                } else {
                    false
                };

                if is_fork {
                    warn!("⚠️  FORK DETECTED at height {}!", height);
                    warn!(
                        "   Our prev block hash: {:?}",
                        self.storage
                            .get_block_by_height(height - 1)
                            .ok()
                            .flatten()
                            .map(|b| b.hash())
                    );
                    warn!("   Their prev hash: {:?}", block.header.previous_block_hash);

                    // Check if incoming chain is longer (we only have current_height, they have height)
                    if height > current_height {
                        warn!(
                            "🔄 Incoming chain is longer ({} vs {}). SWITCHING TO LONGEST CHAIN!",
                            height, current_height
                        );

                        // Find common ancestor by going backwards
                        let mut common_height = height - 1;
                        while common_height > 0 {
                            if let Ok(Some(_our_block)) =
                                self.storage.get_block_by_height(common_height)
                            {
                                // We have this block, this is our common ancestor
                                info!("✅ Found common ancestor at height {}", common_height);
                                break;
                            }
                            common_height -= 1;
                        }

                        // Rollback: Delete our blocks from common_height+1 to current_height
                        if common_height < current_height {
                            warn!(
                                "🔄 Rolling back blocks {} to {}",
                                common_height + 1,
                                current_height
                            );
                            // Note: We don't have a delete_block method yet, so we'll rebuild WorldState from scratch
                        }

                        // Rebuild WorldState from genesis
                        warn!(
                            "🔄 Rebuilding WorldState from genesis (replaying {} blocks)...",
                            common_height
                        );
                        let mut state = self.state.write().await;
                        *state = WorldState::new(); // Reset to genesis

                        // Credit initial testnet stake to our validator (1000 QBT)
                        if self.config.network == "testnet" {
                            let initial_stake = Amount::new(1000 * 1_000_000_000_000_000_000);
                            state.credit_balance(&self.validator.address, initial_stake);
                            warn!("💰 Credited initial 1000 QBT stake to our validator");
                        }

                        // Track all addresses that receive transactions (other validators)
                        let mut all_addresses = std::collections::HashSet::new();
                        all_addresses.insert(self.validator.address);

                        // Replay all blocks from 0 to common_height
                        for h in 0..=common_height {
                            if let Ok(Some(old_block)) = self.storage.get_block_by_height(h) {
                                // Apply all transactions
                                for tx in &old_block.transactions {
                                    all_addresses.insert(tx.from);
                                    all_addresses.insert(tx.to);

                                    if let Err(e) = state.transfer(&tx.from, &tx.to, tx.amount) {
                                        debug!("Replay tx in block {}: {}", h, e);
                                    }
                                }
                            }
                        }

                        // Now we need to load the CORRECT balances from the NEW chain's storage
                        // For all addresses we've seen in transactions
                        for address in all_addresses {
                            if let Ok(stored_balance) = self.storage.get_balance(&address) {
                                if !stored_balance.is_zero() {
                                    state.set_balance(address, stored_balance);
                                    debug!("💰 Loaded balance for address {:?}", address);
                                }
                            }
                        }

                        // Persist all balances to storage
                        for (address, balance) in state.get_all_balances() {
                            if let Err(e) = self.storage.set_balance(&address, balance) {
                                warn!("Failed to persist balance during rollback: {}", e);
                            }
                        }

                        drop(state);

                        // Update current height to common ancestor
                        *self.current_height.write().await = common_height;

                        warn!(
                            "✅ Rollback complete. Now at height {} with correct WorldState",
                            common_height
                        );

                        // Announce our new height to peers so they know we rolled back
                        if let Some(ref network) = self.network {
                            let mut net = network.write().await;
                            net.set_local_height(common_height);
                        }

                        // Now we can accept the new block
                    } else {
                        warn!(
                            "⊘ Our chain is longer or equal. Rejecting fork block {}",
                            height
                        );
                        return;
                    }
                }

                // Accept the block (either no fork, or we rolled back)
                // Store the block
                if let Err(e) = self.storage.store_block(&block) {
                    error!("Failed to store block {}: {}", height, e);
                    return;
                }

                // Update state with block transactions
                let mut state = self.state.write().await;
                for tx in &block.transactions {
                    if let Err(e) = state.transfer(&tx.from, &tx.to, tx.amount) {
                        warn!("Failed to apply transaction in block {}: {}", height, e);
                    }
                }

                // Persist all balances
                for (address, balance) in state.get_all_balances() {
                    if let Err(e) = self.storage.set_balance(&address, balance) {
                        warn!("Failed to persist balance for {}: {}", address, e);
                    }
                }
                drop(state);

                // Update current height
                *self.current_height.write().await = height;

                info!("✅ Block {} accepted and stored", height);
            }
            NetworkEvent::NewTransaction(tx) => {
                debug!("📨 Received new transaction from network");

                // Add to mempool if valid
                if let Err(e) = tx.validate() {
                    warn!("Invalid transaction from network: {}", e);
                    return;
                }

                let mut mempool = self.mempool.write().await;
                mempool.push(tx);
            }
            NetworkEvent::BlockRequested(start_height) => {
                // This is actually a range request from GET_BLOCKS:start-end
                // We'll send multiple blocks
                info!("📤 Peer requested blocks starting at {}", start_height);

                // Send up to 50 blocks
                let mut blocks_sent = 0;
                for h in start_height..=(start_height + 50) {
                    if let Ok(Some(block)) = self.storage.get_block_by_height(h) {
                        // Send it via network
                        if let Some(ref network) = self.network {
                            let mut net = network.write().await;
                            if let Err(e) = net.send_block(&block).await {
                                warn!("Failed to send block {}: {}", h, e);
                                break;
                            } else {
                                blocks_sent += 1;
                                debug!("📤 Sent block {}", h);
                            }
                        }
                    } else {
                        // No more blocks available
                        break;
                    }

                    // Small delay to avoid flooding
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }

                if blocks_sent > 0 {
                    info!("✅ Sent {} blocks to peer", blocks_sent);
                }
            }
        }
    }

    async fn check_mempool(&self) {
        let mempool_guard = self.mempool.read().await;
        let size = mempool_guard.len();
        drop(mempool_guard);

        if size > 0 {
            info!("💾 Mempool: {} pending transactions", size);
        }
    }

    async fn print_stats(&self) {
        let height = self.storage.get_chain_height().unwrap_or(0);

        let mempool_guard = self.mempool.read().await;
        let mempool_size = mempool_guard.len();
        drop(mempool_guard);

        let state = self.state.read().await;

        info!("📊 Validator Stats:");
        info!("   Height: {}", height);
        info!("   Blocks produced: {}", self.blocks_produced);
        info!("   Mempool: {} txs", mempool_size);
        info!("   Accounts: {}", state.account_count());
        info!("   Reputation: {:.2}", self.validator.reputation_score);
    }

    pub async fn stop(&self) {
        *self.is_running.write().await = false;
        info!("Stopping validator node...");
    }

    pub fn blocks_produced(&self) -> u64 {
        self.blocks_produced
    }

    pub fn validator_address(&self) -> Address {
        self.validator.address
    }

    pub fn reputation_score(&self) -> f64 {
        self.validator.reputation_score
    }

    pub fn last_block_height(&self) -> u64 {
        self.validator.last_block_height
    }
}
