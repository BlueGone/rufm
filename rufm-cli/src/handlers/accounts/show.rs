use rufm_core::{models::transactions::Transaction, AccountsRepository, TransactionsRepository};

use crate::{handlers::Handler, AccountsShowOpt, Money};

impl Handler for AccountsShowOpt {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        let account = client.get_account_by_name(&self.name)?;
        let balance = client.get_account_balance(&account.id)?;

        struct TransactionData {
            transaction: Transaction,
            is_debit: bool,
        }
        let transactions_data = client
            .get_transactions_for_account(&account.id)?
            .into_iter()
            .map(|transaction| {
                let is_debit = transaction.source_account_id == account.id;

                Ok(TransactionData {
                    transaction,
                    is_debit,
                })
            })
            .collect::<Result<Vec<_>, rufm_core::DatabaseError>>()?;

        println!("{:40} {:6}", account.name, Money(balance));
        println!(" -- ");
        for TransactionData {
            transaction,
            is_debit,
        } in transactions_data
        {
            println!(
                "  {:38} {:8}",
                transaction.name,
                Money(if is_debit {
                    -transaction.amount
                } else {
                    transaction.amount
                })
            );
        }

        Ok(())
    }
}
