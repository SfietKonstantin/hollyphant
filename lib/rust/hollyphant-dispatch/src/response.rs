use crate::ErrorFormatter;
use hollyphant::errors::Error;
use hollyphant::InitialState;
use log::warn;
use serde::Serialize;

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
                warn!("{}", error);

                match error {
                    Error::MasApplicationRegister(instance, error) => {
                        let message = EF::format_error_mas_application_register(&instance);
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
    String(String),
}

impl From<InitialState> for Response {
    fn from(value: InitialState) -> Self {
        Response::InitialState(value)
    }
}

impl From<String> for Response {
    fn from(value: String) -> Self {
        Response::String(value)
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
            let status = Status::success(Response::InitialState(InitialState::NoAccount));
            let status: Value = to_value(status).unwrap();
            let expected = json!({
                "status": "success",
                "value": "no-account"
            });
            assert_eq!(status, expected);
        }
        {
            let status = Status::success(Response::String("string-value".to_string()));
            let status: Value = to_value(status).unwrap();
            let expected = json!({
                "status": "success",
                "value": "string-value"
            });
            assert_eq!(status, expected);
        }
    }
}
