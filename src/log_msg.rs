use std::fmt;
use serde::{Serialize, Deserialize};
use serde_json::Value;


/// Thin wrapper around a JSON value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogMessage(Value);


impl fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Some log message")
    }
}


impl From<LogMessage> for Value {
    fn from(m: LogMessage) -> Self {
        m.0
    }
}


impl From<Value> for LogMessage {
    fn from(v: Value) -> Self {
        Self(v)
    }
}