pub mod errors;

mod account;
mod db;
mod rest;
mod schema;

use crate::account::MasAccountCache;
use diesel::{Connection, ConnectionError, ConnectionResult, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use reqwest::Client;
use serde::Serialize;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum InitialState {
    HasAccount,
    NoAccount,
}

pub struct Hollyphant {
    db_connection: SqliteConnection,
    client: Client,
    mas_account_cache: MasAccountCache,
}

impl Hollyphant {
    pub fn new(db_connection: SqliteConnection, client: Client) -> Self {
        Self {
            db_connection,
            client,
            mas_account_cache: MasAccountCache::default(),
        }
    }

    pub fn initial_state(&self) -> InitialState {
        InitialState::NoAccount
    }
}

pub fn new_database(database_url: &str) -> ConnectionResult<SqliteConnection> {
    let mut connection = SqliteConnection::establish(database_url)?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|error| ConnectionError::BadConnection(error.to_string()))?;
    Ok(connection)
}
