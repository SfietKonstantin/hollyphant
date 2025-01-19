use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};

pub struct MastodonAuthApi<'a> {
    client: &'a Client,
    instance: &'a str,
}

impl<'a> MastodonAuthApi<'a> {
    pub fn new(client: &'a Client, instance: &'a str) -> Self {
        Self { client, instance }
    }

    pub async fn post_application(&self, body: MasApplicationRequest) -> Result<MasApplication> {
        let url = format!("{}/api/v1/apps", self.instance);
        let response = self.client.post(url).json(&body).send().await?;
        let response = response.error_for_status()?;
        response.json().await
    }
}

#[derive(Debug, Serialize)]
pub struct MasApplicationRequest {
    client_name: String,
    redirect_uris: String,
}

impl MasApplicationRequest {
    pub fn new(client_name: String, redirect_uris: String) -> Self {
        MasApplicationRequest {
            client_name,
            redirect_uris,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct MasApplication {
    pub vapid_key: String,
    pub client_id: String,
    pub client_secret: String,
}
