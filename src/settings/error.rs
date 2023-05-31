use std::{fmt, error};
use mongodb;

use kirkiano_util::net::port;


/// Settings error
#[derive(Debug, Clone)]
pub enum Error {
    /// No listen port specified
    NoPort,

    /// Failed to parse string into a port (pos. integer)
    NotAPort(String, port::Error),

    /// Failed to parse string into a channel capacity (pos. integer)
    NotAChannelCapacity(String),

    /// No Mongo URI provided
    NoMongoURI,

    /// The given Mongo URI is invalid, for the given reason
    InvalidMongoURI(String, mongodb::error::Error),

    /// No Mongo database specified
    NoMongoDBName,
}


impl error::Error for Error {}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {

            Self::NoPort =>
                write!(f, "No port specified"),

            Self::NotAPort(s, e) =>
                write!(f, "Not a port '{}': {}", s, e),

            Self::NoMongoURI =>
                write!(f, "MongoDB URI not provided"),

            Self::InvalidMongoURI(s, e) =>
                write!(f, "MongoDB URI '{}' is invalid: {}", s, e),

            Self::NoMongoDBName =>
                write!(f, "MongDB DB name unspecified"),

            Self::NotAChannelCapacity(s) =>
                write!(f, "Not a channel capacity '{}'", s),
        }
    }
}
