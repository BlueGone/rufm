use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql};
use diesel::sql_types::Integer;
use std::convert::TryFrom;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type = "Integer"]
pub enum AccountType {
    Asset,
    Expense,
    Revenue,
}

impl<DB> FromSql<Integer, DB> for AccountType
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        AccountType::try_from(i32::from_sql(bytes)?).map_err(|e| e.into())
    }
}

impl<DB> ToSql<Integer, DB> for AccountType
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: std::io::Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        i32::to_sql(&(*self).into(), out)
    }
}

impl TryFrom<i32> for AccountType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccountType::Asset),
            1 => Ok(AccountType::Expense),
            2 => Ok(AccountType::Revenue),
            _ => Err("Conversion failed"),
        }
    }
}

impl From<AccountType> for i32 {
    fn from(value: AccountType) -> Self {
        value as Self
    }
}
