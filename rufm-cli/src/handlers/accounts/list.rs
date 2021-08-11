use rufm_core::AccountsRepository;

use crate::{handlers::Handler, AccountsListOpt, Money};

impl Handler for AccountsListOpt {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        let accounts = client.list_asset_accounts()?;

        let accounts_with_balance = accounts
            .iter()
            .map(|account| {
                let balance = client.get_account_balance(&account.id)?;

                Ok((account, balance))
            })
            .collect::<Result<Vec<_>, rufm_core::DatabaseError>>()?;

        for (account, balance) in accounts_with_balance {
            println!("{:60} {}", account.name, Money(balance));
        }

        Ok(())
    }
}
