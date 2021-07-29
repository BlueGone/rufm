#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_newtype;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!("migrations");

pub mod models;
pub mod schema;

use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use models::accounts::*;
use models::transactions::*;

pub struct Client {
    conn: SqliteConnection,
}

impl Client {
    pub fn new(file: Option<&str>) -> Result<Client, Box<dyn std::error::Error>> {
        let conn = SqliteConnection::establish(match file {
            Some(path) => path,
            None => ":memory:",
        })?;
        
        diesel_migrations::run_pending_migrations(&conn)?;
        
        Ok(Client { conn })
    }
}

type RepositoryResult<T> = Result<T, diesel::result::Error>;

pub trait TransactionsRepository {
    fn create_transaction(&self, new_transaction: &NewTransaction)
        -> RepositoryResult<Transaction>;
}

pub trait AccountsRepository {
    fn create_account(&self, new_account: &NewAccount) -> RepositoryResult<Account>;
}

impl AccountsRepository for Client {
    fn create_account(
        &self,
        new_account: &models::accounts::NewAccount,
    ) -> RepositoryResult<Account> {
        insert_into(schema::accounts::table)
            .values(new_account)
            .execute(&self.conn)?;

        schema::accounts::table
            .order(schema::accounts::id.desc())
            .first::<Account>(&self.conn)
    }
}

impl TransactionsRepository for Client {
    fn create_transaction(
        &self,
        new_transaction: &models::transactions::NewTransaction,
    ) -> RepositoryResult<Transaction> {
        insert_into(schema::transactions::table)
            .values(new_transaction)
            .execute(&self.conn)?;

        schema::transactions::table
            .order(schema::transactions::id.desc())
            .first::<Transaction>(&self.conn)
    }
}
