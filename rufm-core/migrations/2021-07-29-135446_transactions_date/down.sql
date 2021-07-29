ALTER TABLE transactions
RENAME TO transactions_old;

CREATE TABLE transactions (
    id integer NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    source_account_id int NOT NULL,
    destination_account_id int NOT NULL,
    amount bigint NOT NULL,

    FOREIGN KEY (source_account_id) REFERENCES accounts (id),
    FOREIGN KEY (destination_account_id) REFERENCES accounts (id)
);

INSERT INTO transactions (
    id,
    name,
    source_account_id,
    destination_account_id,
    amount
)
SELECT
    id,
    name,
    source_account_id,
    destination_account_id,
    amount
FROM transactions_old;

DROP TABLE transactions_old;
