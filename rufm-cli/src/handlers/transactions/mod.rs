use crate::TransactionsCommand;

mod create;
mod list;

pub fn handle_transactions_command(
    client: &rufm_core::Client,
    transactions_command: TransactionsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match transactions_command {
        TransactionsCommand::Create {
            name,
            amount,
            source_account,
            destination_account,
        } => create::handle_transactions_create_command(
            client,
            &name,
            &amount,
            &source_account,
            &destination_account,
        ),
        TransactionsCommand::List => list::handle_transactions_list_command(client),
    }
}
