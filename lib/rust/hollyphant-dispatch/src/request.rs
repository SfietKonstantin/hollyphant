use log::warn;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "context")]
pub enum Request {
    Init,
    NewAccount(RequestNewAccount),
}

impl Request {
    pub fn try_parse(key: &[u8], args: &[u8]) -> Option<Self> {
        let key = try_deserialize_slice(key, "key")?;
        let args = try_deserialize_slice(args, "args")?;

        let input = match key {
            Value::Object(mut object) => {
                object.insert("args".to_string(), args);
                Value::Object(object)
            }
            _ => key,
        };

        match serde_json::from_value(input) {
            Ok(request) => Some(request),
            Err(error) => {
                warn!("Could not deserialize request from input {}", error);
                None
            }
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "service")]
pub enum RequestNewAccount {
    Mastodon(RequestNewAccountMastodon),
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "action")]
pub enum RequestNewAccountMastodon {
    OpenBrowser { args: String },
}

fn try_deserialize_slice(slice: &[u8], kind: &str) -> Option<Value> {
    match serde_json::from_slice(slice) {
        Ok(value) => Some(value),
        Err(error) => {
            warn!("Could not deserialize {kind}: {error}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_value, json};

    #[test]
    fn deserialize_keys() {
        {
            let key = json!({ "context": "init" });
            let key: Request = from_value(key).unwrap();
            let expected = Request::Init;
            assert_eq!(key, expected);
        }
        {
            let key = json!({
                "context": "new-account",
                "service": "mastodon",
                "action": "open-browser",
                "args": "https://mastodon.social"
            });
            let key: Request = from_value(key).unwrap();
            let expected = Request::NewAccount(RequestNewAccount::Mastodon(
                RequestNewAccountMastodon::OpenBrowser {
                    args: "https://mastodon.social".to_string(),
                },
            ));
            assert_eq!(key, expected);
        }
    }
}
