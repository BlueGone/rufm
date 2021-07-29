use crate::schema::accounts;

#[derive(Queryable, Debug)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
}
