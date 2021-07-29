use rufm_core::models::accounts::*;
use rufm_core::models::transactions::*;
use rufm_core::*;

#[test]
fn can_create_account() {
    let client = Client::new(None).unwrap();
    let new_account = NewAccount { name: "test" };

    let actual = client.create_account(&new_account).unwrap();

    assert_eq!(new_account.name, actual.name);
}

#[test]
fn balance_is_zero_after_creation() {
    let client = Client::new(None).unwrap();
    let account = client.create_account(&NewAccount { name: "test" }).unwrap();

    let balance = client.get_account_balance(&account.id).unwrap();

    assert_eq!(0, balance);
}

#[test]
fn spending_money_debits_the_given_amount() {
    let client = Client::new(None).unwrap();
    let account = client
        .create_account(&NewAccount { name: "source" })
        .unwrap();
    let destination_account = client
        .create_account(&NewAccount {
            name: "destination",
        })
        .unwrap();
    let amount = 100;
    client
        .create_transaction(&NewTransaction {
            name: "transaction",
            source_account_id: account.id,
            destination_account_id: destination_account.id,
            amount: amount,
        })
        .unwrap();

    let balance = client.get_account_balance(&account.id).unwrap();

    assert_eq!(balance, -amount);
}

#[test]
fn receiving_money_debits_the_given_amount() {
    let client = Client::new(None).unwrap();
    let account = client
        .create_account(&NewAccount { name: "source" })
        .unwrap();
    let source_account = client
        .create_account(&NewAccount { name: "source" })
        .unwrap();
    let amount = 100;
    client
        .create_transaction(&NewTransaction {
            name: "transaction",
            source_account_id: source_account.id,
            destination_account_id: account.id,
            amount: amount,
        })
        .unwrap();

    let balance = client.get_account_balance(&account.id).unwrap();

    assert_eq!(balance, amount);
}
