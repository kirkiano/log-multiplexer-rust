use std::{fmt, error, io};
use mongodb;

use crate::settings;


/// Top-level error
#[derive(Debug)]
pub enum Error {
    Settings(settings::Error),
    CannotConnectToMongo(mongodb::error::Error),
    CannotBind(io::Error),
    ListenFailure,
    CannotAcceptClient(io::Error),
    ClientDeparted(String, i32),
    CannotRecv,
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            Self::Settings(e) =>
                e.fmt(f),

            Self::CannotConnectToMongo(e) =>
                write!(f, "Cannot connecto to MongoDB: {}", e),

            Self::CannotBind(e) =>
                write!(f, "Cannot bind: {}", e),

            Self::ListenFailure =>
                write!(f, "Cannot listen for more clients"),

            Self::CannotAcceptClient(e) =>
                write!(f, "Cannot accept client: {}", e),

            Self::ClientDeparted(desc, e) =>
                write!(f, "{} has departed: {}", desc, e),

            Self::CannotRecv =>
                write!(f, "Cannot receive"),
        }
    }
}

impl error::Error for Error {}


impl Error {
    pub fn is_listen_failure(&self) -> bool {
        matches!(self, Self::ListenFailure)
    }
}