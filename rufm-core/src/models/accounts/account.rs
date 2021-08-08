use super::{AccountId, AccountType};

#[derive(Queryable, Debug, Hash, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub name: String,
    pub account_type: AccountType,
}
