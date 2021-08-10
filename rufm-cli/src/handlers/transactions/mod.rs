use crate::{handlers::Handler, TransactionsCommand, TransactionsListOpt};

mod create;
mod list;

impl Handler for TransactionsCommand {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            TransactionsCommand::Create(transactions_create_opt) => {
                transactions_create_opt.handle(client)
            }
            TransactionsCommand::List => TransactionsListOpt.handle(client),
        }
    }
}
