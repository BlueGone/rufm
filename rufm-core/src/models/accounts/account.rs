use super::{AccountId, AccountType};
use crate::schema::accounts;

#[derive(Queryable, Identifiable, Debug, Hash, PartialEq, Eq)]
#[table_name = "accounts"]
pub struct Account {
    pub id: AccountId,
    pub name: String,
    pub account_type: AccountType,
    pub initial_balance: i64,
}
