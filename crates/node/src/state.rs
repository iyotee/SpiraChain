use spirachain_core::{Address, Amount, Result, SpiraChainError};
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
        self.accounts
            .get(address)
            .map(|acc| acc.balance)
            .unwrap_or(Amount::zero())
    }

    pub fn set_balance(&mut self, address: Address, balance: Amount) {
        self.accounts
            .entry(address)
            .or_insert(AccountState {
                balance: Amount::zero(),
                nonce: 0,
                stake: Amount::zero(),
            })
            .balance = balance;
    }

    pub fn credit_balance(&mut self, address: &Address, amount: Amount) {
        let current_balance = self.get_balance(address);
        if let Some(new_balance) = current_balance.checked_add(amount) {
            self.set_balance(*address, new_balance);
        }
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

        Err(SpiraChainError::InsufficientBalance)
    }

    pub fn get_nonce(&self, address: &Address) -> u64 {
        self.accounts.get(address).map(|acc| acc.nonce).unwrap_or(0)
    }

    pub fn increment_nonce(&mut self, address: &Address) {
        self.accounts
            .entry(*address)
            .or_insert(AccountState {
                balance: Amount::zero(),
                nonce: 0,
                stake: Amount::zero(),
            })
            .nonce += 1;
    }

    pub fn get_stake(&self, address: &Address) -> Amount {
        self.accounts
            .get(address)
            .map(|acc| acc.stake)
            .unwrap_or(Amount::zero())
    }

    pub fn add_stake(&mut self, address: &Address, amount: Amount) -> Result<()> {
        let balance = self.get_balance(address);

        if let Some(new_balance) = balance.checked_sub(amount) {
            let acc = self.accounts.entry(*address).or_insert(AccountState {
                balance: Amount::zero(),
                nonce: 0,
                stake: Amount::zero(),
            });

            acc.balance = new_balance;
            acc.stake = acc
                .stake
                .checked_add(amount)
                .ok_or_else(|| SpiraChainError::Internal("Stake overflow".to_string()))?;

            Ok(())
        } else {
            Err(SpiraChainError::InsufficientBalance)
        }
    }

    pub fn remove_stake(&mut self, address: &Address, amount: Amount) -> Result<()> {
        let acc = self
            .accounts
            .get_mut(address)
            .ok_or_else(|| SpiraChainError::ValidatorNotFound(address.to_string()))?;

        if let Some(new_stake) = acc.stake.checked_sub(amount) {
            acc.stake = new_stake;
            acc.balance = acc
                .balance
                .checked_add(amount)
                .ok_or_else(|| SpiraChainError::Internal("Balance overflow".to_string()))?;
            Ok(())
        } else {
            Err(SpiraChainError::InsufficientStake(
                acc.stake.value(),
                amount.value(),
            ))
        }
    }

    pub fn current_height(&self) -> u64 {
        self.block_height
    }

    pub fn set_height(&mut self, height: u64) {
        self.block_height = height;
    }

    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    pub fn total_supply(&self) -> Amount {
        self.accounts
            .values()
            .map(|acc| acc.balance.checked_add(acc.stake).unwrap_or(acc.balance))
            .fold(Amount::zero(), |sum, balance| {
                sum.checked_add(balance).unwrap_or(sum)
            })
    }

    pub fn total_staked(&self) -> Amount {
        self.accounts
            .values()
            .map(|acc| acc.stake)
            .fold(Amount::zero(), |sum, stake| {
                sum.checked_add(stake).unwrap_or(sum)
            })
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}
