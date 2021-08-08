use super::AccountType;
use crate::schema::accounts;

#[derive(Insertable, Debug)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub account_type: AccountType,
    pub initial_balance: i64,
}
