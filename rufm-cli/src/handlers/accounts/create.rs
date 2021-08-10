use rufm_core::{
    models::accounts::{AccountType, NewAccount},
    AccountsRepository,
};

use crate::{handlers::Handler, AccountsCreateOpt};

impl Handler for AccountsCreateOpt {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        client.create_account(&NewAccount {
            name: &self.name,
            account_type: AccountType::Asset,
            initial_balance: self.initial_balance.0,
        })?;

        Ok(())
    }
}
