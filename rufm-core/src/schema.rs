table! {
    accounts (id) {
        id -> Integer,
        name -> Text,
        account_type -> Integer,
        initial_balance -> BigInt,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        name -> Text,
        source_account_id -> Integer,
        destination_account_id -> Integer,
        amount -> BigInt,
        date -> Date,
    }
}

allow_tables_to_appear_in_same_query!(accounts, transactions,);
