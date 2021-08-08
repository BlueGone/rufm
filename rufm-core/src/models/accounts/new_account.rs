use crate::schema::accounts;

#[derive(Insertable, Debug)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
}
