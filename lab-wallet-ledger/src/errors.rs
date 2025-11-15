use std::fmt;

use crate::domain::{Money, WalletId};

#[derive(Debug)]
pub enum DomainError {
    WalletNotFound(WalletId),
    InsufficientFunds {
        balance: Money,
        attempted: Money,
    },
    InvalidAmount(Money),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::WalletNotFound(id) => {
                write!(f, "wallet not found: {}", id.0)
            }
            DomainError::InsufficientFunds { balance, attempted } => {
                write!(
                    f,
                    "insufficient funds: balance={}, attempted={}",
                    balance, attempted
                )
            }
            DomainError::InvalidAmount(amount) => {
                write!(f, "invalid amount: {}", amount)
            }
        }
    }
}

impl std::error::Error for DomainError {}
