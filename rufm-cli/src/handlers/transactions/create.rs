use crate::handlers::Handler;
use crate::TransactionsCreateOpt;
use rufm_core::models::transactions::NewTransaction;
use rufm_core::{AccountsRepository, TransactionsRepository};

impl Handler for TransactionsCreateOpt {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        let source_account = client.get_account_by_name(&self.source_account)?;
        let destination_account = client.get_account_by_name(&self.destination_account)?;

        client.create_transaction(&NewTransaction {
            name: &self.name,
            amount: self.amount.0,
            source_account_id: source_account.id,
            destination_account_id: destination_account.id,
            date: chrono::Local::now().naive_local().date(),
        })?;

        Ok(())
    }
}
