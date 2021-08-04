use structopt::StructOpt;

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

fn main() {
    let opt: Opt = Opt::from_args();

    println!("{:?}", opt);
}
