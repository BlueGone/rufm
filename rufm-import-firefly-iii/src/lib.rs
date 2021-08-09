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
}

#[derive(Debug, Deserialize)]
pub enum AccountType {
    #[serde(rename = "Asset account")]
    Asset,
    #[serde(rename = "Expense account")]
    Expense,
    #[serde(rename = "Revenue account")]
    Revenue,
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
    _client: &Client,
    _rdr: R,
) -> Result<(), ImportFireflyIiiError> {
    let mut _csv_reader = Reader::from_reader(_rdr);

    for record in _csv_reader
        .deserialize()
        .collect::<Result<Vec<CsvRecord>, csv::Error>>()?
        .iter()
    {
        println!("{:?}", record);
    }

    todo!()
}
