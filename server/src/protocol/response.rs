use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Response {
    Ok {
        #[serde(rename = "type")]
        kind: &'static str,
        #[serde(skip_serializing_if = "Value::is_null")]
        data: Value,
    },
    Error {
        code: u16,
        message: String,
    },
}

impl Response {
    pub fn ok(kind: &'static str, data: Value) -> Self {
        Response::Ok { kind, data }
    }

    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Response::Error { code, message: message.into() }
    }

    pub fn to_line(&self) -> String {
        let body = serde_json::to_string(self).unwrap_or_else(|_| {
            r#"{"status":"error","code":500,"message":"serialization failure"}"#.to_string()
        });
        format!("{}\n", body)
    }
}
