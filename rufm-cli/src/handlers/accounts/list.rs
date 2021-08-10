use crate::{handlers::Handler, AccountsListOpt, Money};
use rufm_core::AccountsRepository;

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
