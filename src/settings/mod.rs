
mod error;
mod mongo;
mod struc;

pub use error::Error;
pub use mongo::Mongo;
pub use struc::Settings;

pub type Result<T> = std::result::Result<T, Error>;