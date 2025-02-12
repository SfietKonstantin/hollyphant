use crate::db::NewMasAccount;
use crate::errors::{Error, Result};
use crate::rest::mastodon::{MasApplicationRequest, MasTokenRequest, MastodonAuthApi};
use crate::Hollyphant;
use diesel::RunQueryDsl;
use std::collections::HashMap;
use tracing::{event, Level};

const REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

#[derive(Default)]
pub struct MasAccountCache {
    cache: HashMap<String, MasAccount>,
}

impl Hollyphant {
    pub async fn mas_pre_login(&mut self, name: String, instance: String) -> Result<String> {
        let api = MastodonAuthApi::new(&self.client, &instance);

        let application =
            MasApplicationRequest::new("Hollyphant".to_string(), vec![REDIRECT_URI.to_string()]);
        let app = api
            .post_application(application)
            .await
            .map_err(|error| Error::MasApplicationRegister(instance.to_string(), error))?;

        let url = format!(
            "{instance}/oauth/authorize?response_type=code&client_id={}&redirect_uri={REDIRECT_URI}&force_login=true",
        app.client_id);
        self.mas_account_cache.cache.insert(
            name,
            MasAccount::new(instance, app.client_id, app.client_secret),
        );
        Ok(url)
    }

    pub async fn mas_login(&mut self, name: String, code: String) -> Result<()> {
        let account = self.mas_account_cache.cache.get(&name).ok_or_else(|| {
            event!(Level::WARN, account = name, "Account not found");
            Error::Unexpected(format!("Account not found for {name}"))
        })?;

        let api = MastodonAuthApi::new(&self.client, &account.instance);
        let token_request = MasTokenRequest::with_code(
            code,
            account.client_id.clone(),
            account.client_secret.clone(),
            REDIRECT_URI.to_string(),
        );
        let token = api
            .get_token(token_request)
            .await
            .map_err(|error| Error::MasApplicationRegister(account.instance.clone(), error))?;

        let account = NewMasAccount::new(
            &name,
            &account.instance,
            &account.client_id,
            &account.client_secret,
            &token.access_token,
        );
        diesel::insert_into(crate::schema::accounts::table)
            .values(&account)
            .execute(&mut self.db_connection)?;

        Ok(())
    }
}

struct MasAccount {
    instance: String,
    client_id: String,
    client_secret: String,
}

impl MasAccount {
    pub fn new(instance: String, client_id: String, client_secret: String) -> Self {
        Self {
            instance,
            client_id,
            client_secret,
        }
    }
}
