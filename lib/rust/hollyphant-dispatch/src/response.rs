use crate::ErrorFormatter;
use hollyphant::errors::Error;
use hollyphant::InitialState;
use serde::Serialize;
use tracing::{event, Level};

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Set(Status),
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "status")]
pub enum Status {
    InProgress,
    Success { value: Response },
    Error(FormattedError),
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct FormattedError {
    message: String,
    details: String,
}

impl Status {
    pub fn success(value: Response) -> Self {
        Self::Success { value }
    }

    pub fn from_result<EF>(result: Result<Response, Error>) -> Status
    where
        EF: ErrorFormatter,
    {
        match result {
            Ok(value) => Status::success(value),
            Err(error) => {
                event!(Level::WARN, error = ?error);

                match error {
                    Error::Unexpected(error) => {
                        let message = EF::format_error_unexpected();
                        Status::Error(FormattedError {
                            message,
                            details: error,
                        })
                    }
                    Error::MasApplicationRegister(instance, error) => {
                        let message = EF::format_error_mas_application_register(&instance);
                        Status::Error(FormattedError {
                            message,
                            details: error.to_string(),
                        })
                    }
                    Error::DatabaseError(error) => {
                        let message = EF::format_error_database();
                        Status::Error(FormattedError {
                            message,
                            details: error.to_string(),
                        })
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Response {
    InitialState(InitialState),
    MasOAuthUrl(String),
    AccountCreated(),
}

impl From<InitialState> for Response {
    fn from(value: InitialState) -> Self {
        Response::InitialState(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value, Value};

    #[test]
    fn serialize_status() {
        {
            let status = Status::Error(FormattedError {
                message: "test-message".to_string(),
                details: "test-details".to_string(),
            });
            let status: Value = to_value(status).unwrap();
            let expected = json!({
                "status": "error",
                "message": "test-message",
                "details": "test-details"
            });
            assert_eq!(status, expected);
        }
        {
            let status = Status::success(Response::AccountCreated());
            let status: Value = to_value(status).unwrap();
            let expected = json!({
                "status": "success",
                "value": null
            });
            assert_eq!(status, expected);
        }
        {
            let status = Status::success(Response::InitialState(InitialState::NoAccount));
            let status: Value = to_value(status).unwrap();
            let expected = json!({
                "status": "success",
                "value": "no-account"
            });
            assert_eq!(status, expected);
        }
        {
            let status = Status::success(Response::MasOAuthUrl("http://localhost".to_string()));
            let status: Value = to_value(status).unwrap();
            let expected = json!({
                "status": "success",
                "value": "http://localhost"
            });
            assert_eq!(status, expected);
        }
    }
}
