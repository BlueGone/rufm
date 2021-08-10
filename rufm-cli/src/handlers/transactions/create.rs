use crate::Money;
use rufm_core::models::transactions::NewTransaction;
use rufm_core::{AccountsRepository, TransactionsRepository};

pub fn handle_transactions_create_command(
    client: &rufm_core::Client,
    name: &str,
    amount: &Money,
    source_account: &str,
    destination_account: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_account = client.get_account_by_name(source_account)?;
    let destination_account = client.get_account_by_name(destination_account)?;

    client.create_transaction(&NewTransaction {
        name,
        amount: amount.0,
        source_account_id: source_account.id,
        destination_account_id: destination_account.id,
        date: chrono::Local::now().naive_local().date(),
    })?;

    Ok(())
}
