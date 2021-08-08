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
        /// Initial balance (in euros)
        #[structopt(short, long, default_value = "0")]
        initial_balance: Money,
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

use std::fmt;
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} €", self.0 as f64 / 100.0)
    }
}

fn handle_accounts_command(
    client: &rufm_core::Client,
    accounts_command: AccountsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match accounts_command {
        AccountsCommand::Create { name, initial_balance } => {
            client.create_account(&NewAccount {
                name: &name,
                account_type: AccountType::Asset,
                initial_balance: initial_balance.0
            })?;

            Ok(())
        }
        AccountsCommand::List => {
            let accounts = client.list_accounts()?;

            for account in accounts {
                println!("{} (initial balance {})", account.name,  Money(account.initial_balance));
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
        TransactionsCommand::List => {
            let transactions = client.list_transactions()?;

            let accounts_by_id =
                transactions
                    .iter()
                    .flat_map(|transaction| {
                        [
                            transaction.source_account_id,
                            transaction.destination_account_id,
                        ]
                    })
                    .collect::<std::collections::HashSet<AccountId>>()
                    .iter()
                    .map(|account_id| {
                        let account = client.get_account_by_id(account_id)?;

                        Ok((*account_id, account))
                    })
                    .collect::<Result<
                        std::collections::HashMap<AccountId, Account>,
                        Box<dyn std::error::Error>,
                    >>()?;

            for transaction in transactions {
                println!("{}  {}", transaction.name, Money(transaction.amount));
                println!(
                    "{} --> {}",
                    accounts_by_id
                        .get(&transaction.source_account_id)
                        .expect("account by id")
                        .name,
                    accounts_by_id
                        .get(&transaction.destination_account_id)
                        .expect("account by id")
                        .name,
                );
                println!();
            }

            Ok(())
        }
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
