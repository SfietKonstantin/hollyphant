use log::{debug, log_enabled, warn, Level};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "context")]
pub enum Request {
    Init,
    NewAccount(NewAccount),
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
pub enum NewAccount {
    Mastodon(MastodonNewAccount),
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "action")]
pub enum MastodonNewAccount {
    Prelogin { args: MastodonPreloginArgs },
    Login { args: MastodonLoginArgs },
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct MastodonPreloginArgs {
    pub name: String,
    pub instance: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct MastodonLoginArgs {
    pub name: String,
    pub code: String,
}

fn try_deserialize_slice(slice: &[u8], kind: &str) -> Option<Value> {
    if log_enabled!(Level::Debug) {
        let slice_str = String::from_utf8_lossy(slice);
        debug!("Deserializing {kind} {slice_str}")
    }

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
                "action": "prelogin",
                "args": {
                    "name": "test",
                    "instance": "https://mastodon.social"
                }
            });
            let key: Request = from_value(key).unwrap();
            let expected =
                Request::NewAccount(NewAccount::Mastodon(MastodonNewAccount::Prelogin {
                    args: MastodonPreloginArgs {
                        name: "test".to_string(),
                        instance: "https://mastodon.social".to_string(),
                    },
                }));
            assert_eq!(key, expected);
        }
        {
            let key = json!({
                "context": "new-account",
                "service": "mastodon",
                "action": "login",
                "args": {
                    "name": "test",
                    "code":"123"
                }
            });
            let key: Request = from_value(key).unwrap();
            let expected = Request::NewAccount(NewAccount::Mastodon(MastodonNewAccount::Login {
                args: MastodonLoginArgs {
                    name: "test".to_string(),
                    code: "123".to_string(),
                },
            }));
            assert_eq!(key, expected);
        }
    }
}
