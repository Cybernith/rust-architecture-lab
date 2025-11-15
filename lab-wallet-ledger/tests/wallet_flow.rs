use lab_wallet_ledger::domain::WalletId;
use lab_wallet_ledger::errors::DomainError;
use lab_wallet_ledger::service::WalletService;

#[test]
fn happy_path_deposit_and_withdraw() {
    let mut service = WalletService::new();
    let wallet_id = WalletId::new("user-1");

    service.create_wallet(wallet_id.clone());
    service.deposit(&wallet_id, 1_000).unwrap();
    service.deposit(&wallet_id, 500).unwrap();
    service.withdraw(&wallet_id, 700).unwrap();

    let balance = service.balance(&wallet_id).unwrap();
    assert_eq!(balance, 800);
}

#[test]
fn cannot_withdraw_more_than_balance() {
    let mut service = WalletService::new();
    let wallet_id = WalletId::new("user-2");

    service.create_wallet(wallet_id.clone());
    service.deposit(&wallet_id, 500).unwrap();

    let result = service.withdraw(&wallet_id, 1_000);
    assert!(matches!(
        result,
        Err(DomainError::InsufficientFunds { .. })
    ));

    let balance = service.balance(&wallet_id).unwrap();
    assert_eq!(balance, 500);
}

#[test]
fn invalid_amount_rejected() {
    let mut service = WalletService::new();
    let wallet_id = WalletId::new("user-3");

    service.create_wallet(wallet_id.clone());

    let res1 = service.deposit(&wallet_id, 0);
    let res2 = service.withdraw(&wallet_id, -10);

    assert!(matches!(res1, Err(DomainError::InvalidAmount(_))));
    assert!(matches!(res2, Err(DomainError::InvalidAmount(_))));
}
