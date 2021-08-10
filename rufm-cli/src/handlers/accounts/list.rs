use crate::Money;
use rufm_core::AccountsRepository;

pub fn handle_accounts_list_command(
    client: &rufm_core::Client,
) -> Result<(), Box<dyn std::error::Error>> {
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
