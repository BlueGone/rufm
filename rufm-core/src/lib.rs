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

impl Client {
    pub fn new(file: Option<&str>) -> Result<Client, Box<dyn std::error::Error>> {
        let conn = SqliteConnection::establish(file.unwrap_or(":memory:"))?;

        embedded_migrations::run(&conn)?;

        Ok(Client { conn })
    }
}

type RepositoryResult<T> = Result<T, diesel::result::Error>;

pub trait TransactionsRepository {
    fn create_transaction(&self, new_transaction: &NewTransaction)
        -> RepositoryResult<Transaction>;
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

    fn get_account_balance(&self, account_id: &AccountId) -> RepositoryResult<i64> {
        self.get_transactions_for_account(account_id)
            .map(|transactions| get_account_balance_from_transactions(transactions, account_id))
    }

    fn get_account_balance_as_of_date(
        &self,
        account_id: &AccountId,
        date: &chrono::NaiveDate,
    ) -> RepositoryResult<i64> {
        self.get_transactions_for_account_before_date_included(account_id, date)
            .map(|transactions| get_account_balance_from_transactions(transactions, account_id))
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
            .get_results(&self.conn)
    }
}
