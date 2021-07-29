mod database;

use diesel::prelude::*;
use diesel::dsl::*;
use std::error::Error;
use rufm::models::accounts::*;
use rufm::models::transactions::*;
use rufm::schema::accounts;
use rufm::schema::transactions;

#[test]
fn can_create_transaction() -> Result<(), Box<dyn Error>> {
    let conn = database::setup()?;

    insert_into(accounts::table)
        .values(&vec![
            NewAccount { name: "source" },
            NewAccount {
                name: "destination",
            },
        ])
        .execute(&conn)?;
    let expected = NewTransaction {
        name: "test",
        source_account_id: 1,
        destination_account_id: 2,
        amount: 100,
    };

    insert_into(transactions::table).values(&expected).execute(&conn)?;
    let actual = transactions::table.first::<Transaction>(&conn)?;

    assert_eq!(expected.name, actual.name);
    assert_eq!(expected.source_account_id, actual.source_account_id);
    assert_eq!(
        expected.destination_account_id,
        actual.destination_account_id
    );
    assert_eq!(expected.amount, actual.amount);

    Ok(())
}