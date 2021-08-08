use rufm_core::models::accounts::*;
use rufm_core::models::transactions::*;
use rufm_core::*;

#[test]
fn can_create_transaction() {
    let client = Client::new(None).unwrap();
    let source_account = client
        .create_account(&NewAccount {
            name: "source",
            account_type: AccountType::Asset,
        })
        .unwrap();
    let destination_account = client
        .create_account(&NewAccount {
            name: "destination",
            account_type: AccountType::Asset,
        })
        .unwrap();

    let expected = NewTransaction {
        name: "test",
        source_account_id: source_account.id,
        destination_account_id: destination_account.id,
        amount: 100,
        date: chrono::NaiveDate::from_ymd(1970, 1, 1),
    };

    let actual = client.create_transaction(&expected).unwrap();

    assert_eq!(expected.name, actual.name);
    assert_eq!(expected.source_account_id, actual.source_account_id);
    assert_eq!(
        expected.destination_account_id,
        actual.destination_account_id
    );
    assert_eq!(expected.amount, actual.amount);
}
