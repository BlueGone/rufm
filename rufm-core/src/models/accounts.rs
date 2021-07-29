use crate::schema::accounts;

#[derive(DieselNewType, Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct AccountId(pub i32);

#[derive(Queryable, Debug)]
pub struct Account {
    pub id: AccountId,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
}
