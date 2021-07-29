use chrono::Duration;
use chrono::NaiveDate;

use rufm_core::models::accounts::*;
use rufm_core::models::transactions::*;
use rufm_core::*;

#[test]
fn can_create_account() {
    let client = Client::new(None).unwrap();
    let new_account = NewAccount { name: "test" };

    let actual = client.create_account(&new_account).unwrap();

    assert_eq!(new_account.name, actual.name);
}

#[test]
fn balance_is_zero_after_creation() {
    let client = Client::new(None).unwrap();
    let account = client.create_account(&NewAccount { name: "test" }).unwrap();

    let balance = client.get_account_balance(&account.id).unwrap();

    assert_eq!(0, balance);
}

#[test]
fn spending_money_debits_the_given_amount() {
    let (client, account, other_account) = setup_two_accounts().unwrap();

    let amount = 100;
    client
        .create_transaction(&NewTransaction {
            name: "transaction",
            source_account_id: account.id,
            destination_account_id: other_account.id,
            amount: amount,
            date: get_first_day(),
        })
        .unwrap();

    let balance = client.get_account_balance(&account.id).unwrap();

    assert_eq!(balance, -amount);
}

#[test]
fn receiving_money_credits_the_given_amount() {
    let (client, account, other_account) = setup_two_accounts().unwrap();

    let amount = 100;
    client
        .create_transaction(&NewTransaction {
            name: "transaction",
            source_account_id: other_account.id,
            destination_account_id: account.id,
            amount: amount,
            date: get_first_day(),
        })
        .unwrap();

    let balance = client.get_account_balance(&account.id).unwrap();

    assert_eq!(balance, amount);
}

#[test]
fn balance_as_of_before_first_transaction_is_zero() {
    let (client, account, _, _) = setup_two_accounts_and_multiple_transactions().unwrap();

    let balance = client
        .get_account_balance_as_of_date(&account.id, &(get_first_day() - Duration::days(1)))
        .unwrap();

    assert_eq!(balance, 0);
}

#[test]
fn balance_middle_transactions_is_valid() {
    let (client, account, _, _) = setup_two_accounts_and_multiple_transactions().unwrap();

    let balance = client
        .get_account_balance_as_of_date(&account.id, &(get_first_day() + Duration::days(3)))
        .unwrap();

    assert_eq!(balance, -165);
}

#[test]
fn balance_after_transactions_is_valid() {
    let (client, account, _, _) = setup_two_accounts_and_multiple_transactions().unwrap();

    let balance = client
        .get_account_balance_as_of_date(&account.id, &(get_first_day() + Duration::days(10)))
        .unwrap();

    assert_eq!(balance, -191);
}

// Helper functions

fn get_first_day() -> NaiveDate {
    return NaiveDate::from_ymd(1970, 1, 1);
}

fn setup_two_accounts() -> Result<(Client, Account, Account), Box<dyn std::error::Error>> {
    let client = Client::new(None)?;
    let main_account = client.create_account(&NewAccount { name: "main" })?;
    let other_account = client.create_account(&NewAccount { name: "other" })?;

    Ok((client, main_account, other_account))
}

fn setup_two_accounts_and_multiple_transactions(
) -> Result<(Client, Account, Account, Vec<Transaction>), Box<dyn std::error::Error>> {
    let (client, main_account, other_account) = setup_two_accounts()?;

    let transactions = vec![
        (100, true, 0),
        (40, false, 1),
        (12, false, 3),
        (117, true, 3),
        (26, true, 4),
    ]
    .into_iter()
    .map(|(amount, is_debit, days_offset)| {
        let (source_account_id, destination_account_id) = if is_debit {
            (main_account.id, other_account.id)
        } else {
            (other_account.id, main_account.id)
        };

        client.create_transaction(&NewTransaction {
            name: "transaction",
            source_account_id,
            destination_account_id,
            amount,
            date: get_first_day() + Duration::days(days_offset),
        })
    })
    .collect::<Result<Vec<Transaction>, diesel::result::Error>>()?;

    Ok((client, main_account, other_account, transactions))
}
