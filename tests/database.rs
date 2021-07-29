extern crate diesel;
extern crate diesel_migrations;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::error::Error;

pub fn setup() -> Result<SqliteConnection, Box<dyn Error>> {
    let conn = SqliteConnection::establish(":memory:")?;

    diesel_migrations::run_pending_migrations(&conn)?;

    Ok(conn)
}
