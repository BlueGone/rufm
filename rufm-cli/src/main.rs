use handlers::Handler;
use structopt::StructOpt;

mod handlers;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Database file path
    #[structopt(short, long, default_value = "~/.rufm.db")]
    database_path: String,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Create, list, and manage accounts
    #[structopt()]
    Accounts(AccountsCommand),
    /// Create, list, and manage transactions
    #[structopt()]
    Transactions(TransactionsCommand),
    #[cfg(feature = "import-firefly-iii")]
    /// Import from Firefly III
    #[structopt()]
    Import(ImportCommand),
}

#[derive(Debug, StructOpt)]
pub struct AccountsCreateOpt {
    /// Account name
    name: String,
    /// Initial balance (in euros)
    #[structopt(short, long, default_value = "0")]
    initial_balance: Money,
}

pub struct AccountsListOpt;

#[derive(Debug, StructOpt)]
pub struct AccountsShowOpt {
    /// Account name
    name: String,
}

#[derive(Debug, StructOpt)]
pub enum AccountsCommand {
    /// Create an account
    #[structopt()]
    Create(AccountsCreateOpt),
    /// List all account
    #[structopt()]
    List,
    /// Show an account
    #[structopt()]
    Show(AccountsShowOpt),
}

#[derive(Debug, StructOpt)]
pub struct TransactionsCreateOpt {
    /// Account name
    name: String,
    /// Transaction amount (in euros)
    amount: Money,
    /// Source account name
    source_account: String,
    /// Destination account name
    destination_account: String,
}

pub struct TransactionsListOpt;

#[derive(Debug, StructOpt)]
pub enum TransactionsCommand {
    /// Create a transaction
    #[structopt()]
    Create(TransactionsCreateOpt),
    /// List all transactions
    #[structopt()]
    List,
}

#[cfg(feature = "import-firefly-iii")]
#[derive(Debug, StructOpt)]
pub enum ImportCommand {
    /// Import from Firefly III
    #[structopt()]
    FireflyIii {
        /// .csv export file path
        #[structopt()]
        export_file: String,
    },
}

#[derive(Debug)]
pub struct Money(pub i64);

use std::str::FromStr;
impl FromStr for Money {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = f64::from_str(s)?;

        Ok(Money((amount * 100.0).round() as i64))
    }
}

use std::fmt;
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let is_positive = self.0 >= 0;
        let abs_amount = self.0.abs() as f64 / 100.0;

        let uncolored = format!(
            "{} {:7.2} €",
            if is_positive { "+" } else { "-" },
            abs_amount,
        );

        use colored::*;

        write!(
            f,
            "{}",
            if is_positive {
                uncolored.green()
            } else {
                uncolored.red()
            },
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    let database_path = shellexpand::tilde(&opt.database_path);

    let client = rufm_core::Client::new(Some(&database_path))?;

    opt.command.handle(&client)
}
