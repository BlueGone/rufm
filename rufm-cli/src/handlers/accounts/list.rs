use rufm_core::AccountsRepository;

use crate::{handlers::Handler, AccountsListOpt, Money};

impl Handler for AccountsListOpt {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        let accounts = client.list_accounts()?;

        for account in accounts {
            println!(
                "{} (initial balance {})",
                account.name,
                Money(account.initial_balance)
            );
        }

        Ok(())
    }
}
