use super::account_id::AccountId;

#[derive(Queryable, Debug, Hash, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub name: String,
}
