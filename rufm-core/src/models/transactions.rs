use crate::schema::transactions;

#[derive(Queryable, Debug)]
pub struct Transaction {
    pub id: i32,
    pub name: String,
    pub source_account_id: i32,
    pub destination_account_id: i32,
    pub amount: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub name: &'a str,
    pub source_account_id: i32,
    pub destination_account_id: i32,
    pub amount: i64,
}
