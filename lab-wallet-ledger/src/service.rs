use std::collections::HashMap;

use crate::domain::{Money, Wallet, WalletEvent, WalletId};
use crate::errors::DomainError;

pub struct WalletService {
    wallets: HashMap<WalletId, Wallet>,
}

impl WalletService {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
        }
    }

    pub fn create_wallet(&mut self, id: WalletId) -> &Wallet {
        self.wallets
            .entry(id.clone())
            .or_insert_with(|| Wallet::new(id))
    }

    pub fn get_wallet(&self, id: &WalletId) -> Result<&Wallet, DomainError> {
        self.wallets
            .get(id)
            .ok_or_else(|| DomainError::WalletNotFound(id.clone()))
    }

    pub fn get_wallet_mut(&mut self, id: &WalletId) -> Result<&mut Wallet, DomainError> {
        self.wallets
            .get_mut(id)
            .ok_or_else(|| DomainError::WalletNotFound(id.clone()))
    }

    pub fn deposit(&mut self, id: &WalletId, amount: Money) -> Result<(), DomainError> {
        let wallet = self.get_wallet_mut(id)?;
        wallet.deposit(amount)
    }

    pub fn withdraw(&mut self, id: &WalletId, amount: Money) -> Result<(), DomainError> {
        let wallet = self.get_wallet_mut(id)?;
        wallet.withdraw(amount)
    }

    pub fn balance(&self, id: &WalletId) -> Result<Money, DomainError> {
        let wallet = self.get_wallet(id)?;
        Ok(wallet.balance())
    }

    pub fn events(&self, id: &WalletId) -> Result<&[WalletEvent], DomainError> {
        let wallet = self.get_wallet(id)?;
        Ok(wallet.events())
    }
}
