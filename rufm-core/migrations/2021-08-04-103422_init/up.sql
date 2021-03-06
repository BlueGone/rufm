CREATE TABLE accounts (
    id integer NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    account_type integer NOT NULL,
    initial_balance bigint NOT NULL
);

CREATE TABLE transactions (
    id integer NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    source_account_id int NOT NULL,
    destination_account_id int NOT NULL,
    amount bigint NOT NULL,
    date DATE NOT NULL,

    FOREIGN KEY (source_account_id) REFERENCES accounts (id),
    FOREIGN KEY (destination_account_id) REFERENCES accounts (id)
);
