use crate::AccountsCommand;

mod create;
mod list;
mod show;

pub fn handle_accounts_command(
    client: &rufm_core::Client,
    accounts_command: AccountsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match accounts_command {
        AccountsCommand::Create {
            name,
            initial_balance,
        } => create::handle_accounts_create_command(client, &name, &initial_balance),
        AccountsCommand::List => list::handle_accounts_list_command(client),
        AccountsCommand::Show { name } => show::handle_accounts_show_command(client, &name),
    }
}
