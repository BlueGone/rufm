extern crate csv;
extern crate rufm_core;
#[macro_use]
extern crate serde;
use csv::Reader;
use rufm_core::Client;

#[derive(Debug)]
pub struct ImportFireflyIiiError;

impl From<csv::Error> for ImportFireflyIiiError {
    fn from(e: csv::Error) -> ImportFireflyIiiError {
        println!("{}", e);
        panic!("{}", e);
    }
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
        let source_account = get_or_create_account(client, &record.source_name, &record.source_type)?;
        let destination_account = get_or_create_account(client, &record.destination_name, &record.destination_type)?;

        println!("{:?}", record);
        println!("{:?}", source_account);
        println!("{:?}", destination_account);
    }

    todo!()
}

use rufm_core::models::accounts::{Account, AccountType as RufmAccountType, NewAccount};
use rufm_core::AccountsRepository;

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
            .map_err(|_| ImportFireflyIiiError)
    })
}
