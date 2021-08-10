use crate::*;

use super::Handler;

mod create;
mod list;
mod show;

impl Handler for AccountsCommand {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            AccountsCommand::Create(accounts_create_opt) => accounts_create_opt.handle(client),
            AccountsCommand::List => AccountsListOpt.handle(client),
            AccountsCommand::Show(accounts_show_opt) => accounts_show_opt.handle(client),
        }
    }
}
