use crate::Money;
use rufm_core::models::accounts::{Account, AccountId};
use rufm_core::{AccountsRepository, TransactionsRepository};

pub fn handle_transactions_list_command(
    client: &rufm_core::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let transactions = client.list_transactions()?;

    let accounts_by_id =
                transactions
                    .iter()
                    .flat_map(|transaction| {
                        [
                            transaction.source_account_id,
                            transaction.destination_account_id,
                        ]
                    })
                    .collect::<std::collections::HashSet<AccountId>>()
                    .iter()
                    .map(|account_id| {
                        let account = client.get_account_by_id(account_id)?;

                        Ok((*account_id, account))
                    })
                    .collect::<Result<
                        std::collections::HashMap<AccountId, Account>,
                        Box<dyn std::error::Error>,
                    >>()?;

    for transaction in transactions {
        println!("{}  {}", transaction.name, Money(transaction.amount));
        println!(
            "{} --> {}",
            accounts_by_id
                .get(&transaction.source_account_id)
                .expect("account by id")
                .name,
            accounts_by_id
                .get(&transaction.destination_account_id)
                .expect("account by id")
                .name,
        );
        println!();
    }

    Ok(())
}
