#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_newtype;
#[macro_use]
extern crate diesel_migrations;
extern crate chrono;

embed_migrations!();

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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientCreationError {
    #[error("Database connection error: {0}")]
    DatabaseConnectionError(#[from] diesel::result::ConnectionError),
    #[error("Cannot run migrations: {0}")]
    MigrationsError(#[from] diesel_migrations::RunMigrationsError),
}

impl Client {
    pub fn new(file: Option<&str>) -> Result<Client, ClientCreationError> {
        let conn = SqliteConnection::establish(file.unwrap_or(":memory:"))?;

        embedded_migrations::run(&conn)?;

        Ok(Client { conn })
    }
}

pub type DatabaseError = diesel::result::Error;

type RepositoryResult<T> = Result<T, DatabaseError>;

pub trait TransactionsRepository {
    fn create_transaction(&self, new_transaction: &NewTransaction)
        -> RepositoryResult<Transaction>;
    fn list_transactions(&self) -> RepositoryResult<Vec<Transaction>>;
    fn get_transactions_for_account(
        &self,
        account_id: &AccountId,
    ) -> RepositoryResult<Vec<Transaction>>;
    fn get_transactions_for_account_before_date_included(
        &self,
        account_id: &AccountId,
        date: &chrono::NaiveDate,
    ) -> RepositoryResult<Vec<Transaction>>;
}

pub trait AccountsRepository {
    fn create_account(&self, new_account: &NewAccount) -> RepositoryResult<Account>;
    fn list_accounts(&self) -> RepositoryResult<Vec<Account>>;
    fn get_account_by_id(&self, account_id: &AccountId) -> RepositoryResult<Account>;
    fn get_account_by_name(&self, account_name: &str) -> RepositoryResult<Account>;
    fn update_account_initial_balance(&self, account: &Account) -> RepositoryResult<Account>;
    fn get_account_balance(&self, account_id: &AccountId) -> RepositoryResult<i64>;
    fn get_account_balance_as_of_date(
        &self,
        account_id: &AccountId,
        date: &chrono::NaiveDate,
    ) -> RepositoryResult<i64>;
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

    fn list_accounts(&self) -> RepositoryResult<Vec<Account>> {
        schema::accounts::table.get_results(&self.conn)
    }

    fn get_account_by_id(&self, account_id: &AccountId) -> RepositoryResult<Account> {
        schema::accounts::table
            .filter(schema::accounts::id.eq(account_id))
            .first::<Account>(&self.conn)
    }

    fn get_account_by_name(&self, account_name: &str) -> RepositoryResult<Account> {
        schema::accounts::table
            .filter(schema::accounts::name.eq(account_name))
            .first::<Account>(&self.conn)
    }

    fn update_account_initial_balance(&self, account: &Account) -> RepositoryResult<Account> {
        update(account)
            .set(schema::accounts::initial_balance.eq(account.initial_balance))
            .execute(&self.conn)?;

        self.get_account_by_id(&account.id)
    }

    fn get_account_balance(&self, account_id: &AccountId) -> RepositoryResult<i64> {
        let initial_balance = self.get_account_by_id(account_id)?.initial_balance;
        let transactions_sum = get_account_balance_from_transactions(
            self.get_transactions_for_account(account_id)?,
            account_id,
        );

        Ok(initial_balance + transactions_sum)
    }

    fn get_account_balance_as_of_date(
        &self,
        account_id: &AccountId,
        date: &chrono::NaiveDate,
    ) -> RepositoryResult<i64> {
        let initial_balance = self.get_account_by_id(account_id)?.initial_balance;
        let transactions_sum = get_account_balance_from_transactions(
            self.get_transactions_for_account_before_date_included(account_id, date)?,
            account_id,
        );

        Ok(initial_balance + transactions_sum)
    }
}

fn get_transactions_amount_sum(transactions: Vec<Transaction>) -> i64 {
    transactions
        .iter()
        .map(|transmission| transmission.amount)
        .sum::<i64>()
}

fn get_account_balance_from_transactions(
    transactions: Vec<Transaction>,
    account_id: &AccountId,
) -> i64 {
    let (debit_transmissions, credit_transmissions): (Vec<Transaction>, Vec<Transaction>) =
        transactions
            .into_iter()
            .partition(|transmission| transmission.source_account_id == *account_id);

    let debit_sum = get_transactions_amount_sum(debit_transmissions);
    let credit_sum = get_transactions_amount_sum(credit_transmissions);
    credit_sum - debit_sum
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

    fn list_transactions(&self) -> RepositoryResult<Vec<Transaction>> {
        schema::transactions::table
            .order(schema::transactions::date.desc())
            .get_results(&self.conn)
    }

    fn get_transactions_for_account(
        &self,
        account_id: &AccountId,
    ) -> RepositoryResult<Vec<Transaction>> {
        schema::transactions::table
            .filter(
                schema::transactions::source_account_id
                    .eq(account_id)
                    .or(schema::transactions::destination_account_id.eq(account_id)),
            )
            .order(schema::transactions::date.desc())
            .get_results(&self.conn)
    }

    fn get_transactions_for_account_before_date_included(
        &self,
        account_id: &AccountId,
        date: &chrono::NaiveDate,
    ) -> RepositoryResult<Vec<Transaction>> {
        schema::transactions::table
            .filter(
                schema::transactions::source_account_id
                    .eq(account_id)
                    .or(schema::transactions::destination_account_id.eq(account_id))
                    .and(schema::transactions::date.le(date)),
            )
            .order(schema::transactions::date.desc())
            .get_results(&self.conn)
    }
}
