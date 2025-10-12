use spirachain_core::{Address, Amount, Result};
use std::collections::HashMap;

pub struct WorldState {
    accounts: HashMap<Address, AccountState>,
    block_height: u64,
}

pub struct AccountState {
    pub balance: Amount,
    pub nonce: u64,
    pub stake: Amount,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            block_height: 0,
        }
    }

    pub fn get_balance(&self, address: &Address) -> Amount {
        self.accounts.get(address)
            .map(|acc| acc.balance)
            .unwrap_or(Amount::zero())
    }

    pub fn set_balance(&mut self, address: Address, balance: Amount) {
        self.accounts.entry(address)
            .or_insert(AccountState {
                balance: Amount::zero(),
                nonce: 0,
                stake: Amount::zero(),
            })
            .balance = balance;
    }

    pub fn transfer(&mut self, from: &Address, to: &Address, amount: Amount) -> Result<()> {
        let from_balance = self.get_balance(from);
        let to_balance = self.get_balance(to);

        if let Some(new_from_balance) = from_balance.checked_sub(amount) {
            if let Some(new_to_balance) = to_balance.checked_add(amount) {
                self.set_balance(*from, new_from_balance);
                self.set_balance(*to, new_to_balance);
                return Ok(());
            }
        }

        Err(spirachain_core::SpiraChainError::InsufficientBalance)
    }

    pub fn get_nonce(&self, address: &Address) -> u64 {
        self.accounts.get(address)
            .map(|acc| acc.nonce)
            .unwrap_or(0)
    }

    pub fn increment_nonce(&mut self, address: &Address) {
        self.accounts.entry(*address)
            .or_insert(AccountState {
                balance: Amount::zero(),
                nonce: 0,
                stake: Amount::zero(),
            })
            .nonce += 1;
    }

    pub fn current_height(&self) -> u64 {
        self.block_height
    }

    pub fn set_height(&mut self, height: u64) {
        self.block_height = height;
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

