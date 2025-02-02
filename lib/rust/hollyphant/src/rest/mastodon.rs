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

    pub async fn get_token(&self, body: MasTokenRequest) -> Result<MasToken> {
        let url = format!("{}/oauth/token", self.instance);
        let response = self.client.post(url).json(&body).send().await?;
        let response = response.error_for_status()?;
        response.json().await
    }
}

#[derive(Debug, Serialize)]
pub struct MasApplicationRequest {
    client_name: String,
    redirect_uris: Vec<String>,
}

impl MasApplicationRequest {
    pub fn new(client_name: String, redirect_uris: Vec<String>) -> Self {
        MasApplicationRequest {
            client_name,
            redirect_uris,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MasTokenRequest {
    grant_type: String,
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}
impl MasTokenRequest {
    pub fn with_code(
        code: String,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        MasTokenRequest {
            grant_type: "authorization_code".to_string(),
            code,
            client_id,
            client_secret,
            redirect_uri,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct MasApplication {
    pub client_id: String,
    pub client_secret: String,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct MasToken {
    pub access_token: String,
}
