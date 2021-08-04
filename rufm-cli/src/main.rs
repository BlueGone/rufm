use structopt::StructOpt;

use rufm_core::models::accounts::*;
use rufm_core::models::transactions::*;
use rufm_core::AccountsRepository;
use rufm_core::TransactionsRepository;

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
}

#[derive(Debug, StructOpt)]
enum AccountsCommand {
    /// Create an account
    #[structopt()]
    Create {
        /// Account name
        name: String,
    },
    /// List all account
    #[structopt()]
    List,
}

#[derive(Debug, StructOpt)]
enum TransactionsCommand {
    /// Create a transaction
    #[structopt()]
    Create {
        /// Account name
        name: String,
        /// Transaction amount (in euros)
        amount: Money,
        /// Source account name
        source_account: String,
        /// Destination account name
        destination_account: String,
    },
    /// List all transactions
    #[structopt()]
    List,
}

#[derive(Debug)]
struct Money(i64);

use std::str::FromStr;
impl FromStr for Money {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = f64::from_str(s)?;

        Ok(Money((amount * 100.0).round() as i64))
    }
}

fn handle_accounts_command(
    client: &rufm_core::Client,
    accounts_command: AccountsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match accounts_command {
        AccountsCommand::Create { name } => {
            client.create_account(&NewAccount { name: &name })?;

            Ok(())
        }
        AccountsCommand::List => {
            let accounts = client.list_accounts()?;

            for account in accounts {
                println!("{}", account.name)
            }

            Ok(())
        }
    }
}

fn handle_transactions_command(
    client: &rufm_core::Client,
    transactions_command: TransactionsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match transactions_command {
        TransactionsCommand::Create {
            name,
            amount,
            source_account,
            destination_account,
        } => {
            let source_account = client.get_account_by_name(&source_account)?;
            let destination_account = client.get_account_by_name(&destination_account)?;

            client.create_transaction(&NewTransaction {
                name: &name,
                amount: amount.0,
                source_account_id: source_account.id,
                destination_account_id: destination_account.id,
                date: chrono::Local::now().naive_local().date(),
            })?;

            Ok(())
        }
        _ => Ok(()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    let database_path = shellexpand::tilde(&opt.database_path);

    let client = rufm_core::Client::new(Some(&database_path))?;

    match opt.command {
        Command::Accounts(accounts_command) => handle_accounts_command(&client, accounts_command),
        Command::Transactions(transactions_command) => {
            handle_transactions_command(&client, transactions_command)
        }
    }
}
