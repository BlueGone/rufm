mod database;

use diesel::prelude::*;
use diesel::dsl::*;
use std::error::Error;
use rufm_core::models::accounts::*;
use rufm_core::schema::accounts;

#[test]
fn can_create_account() -> Result<(), Box<dyn Error>> {
    let conn = database::setup()?;

    let expected = NewAccount { name: "test" };

    insert_into(accounts::table)
        .values(&expected)
        .execute(&conn)?;
    let actual = accounts::table.first::<Account>(&conn)?;

    assert_eq!(expected.name, actual.name);

    Ok(())
}