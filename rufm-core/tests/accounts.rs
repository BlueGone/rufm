use rufm_core::models::accounts::*;
use rufm_core::*;

#[test]
fn can_create_account() {
    let client = Client::new(None).unwrap();
    let new_account = NewAccount { name: "test" };

    let actual = client.create_account(&new_account).unwrap();

    assert_eq!(new_account.name, actual.name);
}
