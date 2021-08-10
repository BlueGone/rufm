use crate::Money;
use rufm_core::models::accounts::{AccountType, NewAccount};
use rufm_core::AccountsRepository;

pub fn handle_accounts_create_command(
    client: &rufm_core::Client,
    name: &str,
    initial_balance: &Money,
) -> Result<(), Box<dyn std::error::Error>> {
    client.create_account(&NewAccount {
        name,
        account_type: AccountType::Asset,
        initial_balance: initial_balance.0,
    })?;

    Ok(())
}
