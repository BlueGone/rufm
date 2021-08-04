use structopt::StructOpt;

use rufm_core::models::accounts::*;
use rufm_core::AccountsRepository;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    let database_path = shellexpand::tilde(&opt.database_path);

    let client = rufm_core::Client::new(Some(&database_path))?;

    match opt.command {
        Command::Accounts(accounts_command) => handle_accounts_command(&client, accounts_command),
    }
}
