pub mod accounts;
#[cfg(feature = "import-firefly-iii")]
pub mod import;
pub mod transactions;

pub use accounts::*;

use crate::Command;

pub trait Handler {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>>;
}

impl Handler for Command {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Accounts(accounts_command) => accounts_command.handle(client),
            Command::Transactions(transactions_command) => transactions_command.handle(client),
            #[cfg(feature = "import-firefly-iii")]
            Command::Import(import_command) => import_command.handle(client),
        }
    }
}
