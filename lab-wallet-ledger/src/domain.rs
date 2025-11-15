use crate::errors::DomainError;

pub type Money = i64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WalletId(pub String);

impl WalletId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletEvent {
    Deposited { amount: Money },
    Withdrawn { amount: Money },
}

#[derive(Debug, Clone)]
pub struct Wallet {
    id: WalletId,
    events: Vec<WalletEvent>,
    balance: Money,
}

impl Wallet {
    pub fn new(id: WalletId) -> Self {
        Self {
            id,
            events: Vec::new(),
            balance: 0,
        }
    }

    pub fn id(&self) -> &WalletId {
        &self.id
    }

    pub fn balance(&self) -> Money {
        self.balance
    }

    pub fn events(&self) -> &[WalletEvent] {
        &self.events
    }

    fn apply_event(&mut self, event: WalletEvent) {
        match &event {
            WalletEvent::Deposited { amount } => {
                self.balance += amount;
            }
            WalletEvent::Withdrawn { amount } => {
                self.balance -= amount;
            }
        }
        self.events.push(event);
    }

    pub fn deposit(&mut self, amount: Money) -> Result<(), DomainError> {
        if amount <= 0 {
            return Err(DomainError::InvalidAmount(amount));
        }

        let event = WalletEvent::Deposited { amount };
        self.apply_event(event);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Money) -> Result<(), DomainError> {
        if amount <= 0 {
            return Err(DomainError::InvalidAmount(amount));
        }
        if self.balance < amount {
            return Err(DomainError::InsufficientFunds {
                balance: self.balance,
                attempted: amount,
            });
        }

        let event = WalletEvent::Withdrawn { amount };
        self.apply_event(event);
        Ok(())
    }
}
