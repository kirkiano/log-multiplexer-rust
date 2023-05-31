//! A multiplexer of incoming log streams. See README.

mod log_msg;
mod error;
pub mod settings;
mod listener;
pub mod server;
pub mod ship_out;

pub use log_msg::LogMessage;
pub use settings::Settings;
pub use error::Error;
pub use listener::Listener;
pub use ship_out::{Stdout, Mongo};


/// Top-level result type
pub type Result<T> = std::result::Result<T, Error>;