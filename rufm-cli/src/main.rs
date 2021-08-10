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
    #[cfg(feature = "import-firefly-iii")]
    /// Import from Firefly III
    #[structopt()]
    Import(ImportCommand),
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
    /// Show an account
    #[structopt()]
    Show {
        /// Account name
        name: String,
    },
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

#[cfg(feature = "import-firefly-iii")]
#[derive(Debug, StructOpt)]
enum ImportCommand {
    /// Import from Firefly III
    #[structopt()]
    FireflyIii {
        /// .csv export file path
        #[structopt()]
        export_file: String,
    },
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

fn handle_accounts_command(
    client: &rufm_core::Client,
    accounts_command: AccountsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match accounts_command {
        AccountsCommand::Create {
            name,
            initial_balance,
        } => {
            client.create_account(&NewAccount {
                name: &name,
                account_type: AccountType::Asset,
                initial_balance: initial_balance.0,
            })?;

            Ok(())
        }
        AccountsCommand::List => {
            let accounts = client.list_accounts()?;

            for account in accounts {
                println!(
                    "{} (initial balance {})",
                    account.name,
                    Money(account.initial_balance)
                );
            }

            Ok(())
        }
        AccountsCommand::Show { name } => {
            let account = client.get_account_by_name(&name)?;
            let balance = client.get_account_balance(&account.id)?;

            struct TransactionData {
                transaction: Transaction,
                is_debit: bool,
            }
            let transactions_data = client
                .get_transactions_for_account(&account.id)?
                .into_iter()
                .map(|transaction| {
                    let is_debit = transaction.source_account_id == account.id;

                    Ok(TransactionData {
                        transaction,
                        is_debit,
                    })
                })
                .collect::<Result<Vec<_>, rufm_core::DatabaseError>>()?;

            println!("{:40} {:6}", account.name, Money(balance));
            println!(" -- ");
            for TransactionData {
                transaction,
                is_debit,
            } in transactions_data
            {
                println!(
                    "  {:38} {:8}",
                    transaction.name,
                    Money(if is_debit {
                        -transaction.amount
                    } else {
                        transaction.amount
                    })
                );
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

#[cfg(feature = "import-firefly-iii")]
fn handle_import_command(
    client: &rufm_core::Client,
    import_command: ImportCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let ImportCommand::FireflyIii { export_file } = import_command;

    let file = std::fs::File::open(&export_file)?;
    rufm_import_firefly_iii::import_firefly_iii(client, &file)?;

    Ok(())
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
        #[cfg(feature = "import-firefly-iii")]
        Command::Import(import_command) => handle_import_command(&client, import_command),
    }
}
