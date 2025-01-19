use crate::errors::{Error, Result};
use crate::rest::mastodon::{MasApplication, MasApplicationRequest, MastodonAuthApi};
use crate::Hollyphant;

const REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

impl Hollyphant {
    pub async fn mas_pre_login(&self, instance: &str) -> Result<String> {
        let api = MastodonAuthApi::new(&self.client, instance);
        let app = get_mas_app_from_client(&api, instance).await?;
        Ok(format!(
            "{instance}/oauth/authorize?response_type=code&client_id={}&redirect_uri={REDIRECT_URI}",
            app.client_id
        ))
    }
}

async fn get_mas_app_from_client<'a>(
    api: &'a MastodonAuthApi<'a>,
    instance: &'a str,
) -> Result<MasApplication> {
    let application =
        MasApplicationRequest::new("Hollyphant".to_string(), REDIRECT_URI.to_string());
    api.post_application(application)
        .await
        .map_err(|error| Error::MasApplicationRegister(instance.to_string(), error))
}
