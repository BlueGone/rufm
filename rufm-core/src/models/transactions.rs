use crate::models::accounts::AccountId;
use crate::schema::transactions;

#[derive(DieselNewType, Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct TransactionId(pub i32);

#[derive(Queryable, Debug)]
pub struct Transaction {
    pub id: TransactionId,
    pub name: String,
    pub source_account_id: AccountId,
    pub destination_account_id: AccountId,
    pub amount: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub name: &'a str,
    pub source_account_id: AccountId,
    pub destination_account_id: AccountId,
    pub amount: i64,
}
