pub mod errors;

mod account;
mod rest;

use reqwest::Client;
use serde::Serialize;

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum InitialState {
    HasAccount,
    NoAccount,
}

pub struct Hollyphant {
    client: Client,
}

impl Hollyphant {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn initial_state(&self) -> InitialState {
        InitialState::NoAccount
    }
}
