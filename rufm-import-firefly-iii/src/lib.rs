extern crate csv;
extern crate rufm_core;
#[macro_use]
extern crate serde;
use csv::Reader;
use rufm_core::{
    models::{
        accounts::{Account, AccountType as RufmAccountType, NewAccount},
        transactions::{NewTransaction, Transaction},
    },
    AccountsRepository,
    Client,
    TransactionsRepository,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImportFireflyIiiError {
    #[error("csv error: {0}")]
    CsvError(#[from] csv::Error),
    #[error("database error: {0}")]
    DatabaseError(#[from] rufm_core::DatabaseError),
}

#[derive(Debug, Deserialize)]
pub enum TransactionType {
    Withdrawal,
    Deposit,
    Transfer,
    #[serde(rename = "Opening balance")]
    OpeningBalance,
}

#[derive(Debug, Deserialize)]
pub enum AccountType {
    #[serde(rename = "Asset account")]
    Asset,
    #[serde(rename = "Expense account")]
    Expense,
    #[serde(rename = "Revenue account")]
    Revenue,
    #[serde(rename = "Loan")]
    Loan,
    #[serde(rename = "Initial balance account")]
    InitialBalance,
}

#[derive(Debug, Deserialize)]
pub struct CsvRecord {
    #[serde(rename = "type")]
    transaction_type: TransactionType,
    amount: f64,
    description: String,
    date: chrono::DateTime<chrono::offset::Utc>,
    source_name: String,
    source_type: AccountType,
    destination_name: String,
    destination_type: AccountType,
}

pub fn import_firefly_iii<R: std::io::Read>(
    client: &Client,
    rdr: R,
) -> Result<(), ImportFireflyIiiError> {
    let mut _csv_reader = Reader::from_reader(rdr);

    for record in _csv_reader
        .deserialize()
        .collect::<Result<Vec<CsvRecord>, csv::Error>>()?
        .iter()
        .rev()
    {
        match record {
            CsvRecord {
                transaction_type: TransactionType::OpeningBalance,
                source_type: AccountType::InitialBalance,
                ..
            } => handle_initial_balance(client, record),
            CsvRecord {
                transaction_type: TransactionType::Withdrawal,
                ..
            } => handle_withdrawal(client, record),
            // We assume that every transaction can be treated the same as a withdrawal.
            _ => handle_withdrawal(client, record),
        }?;
    }

    Ok(())
}

pub fn handle_withdrawal(client: &Client, record: &CsvRecord) -> Result<(), ImportFireflyIiiError> {
    let source_account = get_or_create_account(client, &record.source_name, &record.source_type)?;
    let destination_account =
        get_or_create_account(client, &record.destination_name, &record.destination_type)?;
    let transaction = create_transaction(
        client,
        record.amount,
        &record.description,
        &record.date,
        &source_account,
        &destination_account,
    )?;

    println!(
        "Created transaction '{}' ({}) from '{}' to '{}'",
        transaction.name, transaction.amount, source_account.name, destination_account.name,
    );

    Ok(())
}

pub fn record_amount_to_rufm_amount(amount: f64) -> i64 {
    (amount * 100.0).round() as i64
}

pub fn handle_initial_balance(
    client: &Client,
    record: &CsvRecord,
) -> Result<(), ImportFireflyIiiError> {
    let mut account =
        get_or_create_account(client, &record.destination_name, &record.destination_type)?;

    account.initial_balance = record_amount_to_rufm_amount(-record.amount);

    let new_account = client.update_account_initial_balance(&account)?;
    println!(
        "Updated initial balance of account '{}' to {}",
        new_account.name, new_account.initial_balance
    );

    Ok(())
}

impl From<&AccountType> for RufmAccountType {
    fn from(e: &AccountType) -> RufmAccountType {
        match e {
            AccountType::Asset => RufmAccountType::Asset,
            AccountType::Expense => RufmAccountType::Expense,
            AccountType::Revenue => RufmAccountType::Revenue,
            AccountType::Loan => RufmAccountType::Asset,
            AccountType::InitialBalance => RufmAccountType::Asset,
        }
    }
}

fn get_or_create_account(
    client: &Client,
    account_name: &str,
    account_type: &AccountType,
) -> Result<Account, ImportFireflyIiiError> {
    client.get_account_by_name(account_name).or_else(|_| {
        client
            .create_account(&NewAccount {
                name: account_name,
                account_type: account_type.into(),
                initial_balance: 0,
            })
            .map_err(|e| e.into())
    })
}

fn create_transaction(
    client: &Client,
    amount: f64,
    description: &str,
    date: &chrono::DateTime<chrono::offset::Utc>,
    source_account: &Account,
    destination_account: &Account,
) -> Result<Transaction, ImportFireflyIiiError> {
    client
        .create_transaction(&NewTransaction {
            name: description,
            amount: (-amount * 100.0).round() as i64,
            source_account_id: source_account.id,
            destination_account_id: destination_account.id,
            date: date.naive_utc().date(),
        })
        .map_err(|e| e.into())
}
