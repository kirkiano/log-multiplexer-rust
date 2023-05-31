use std::io::{self, Write};
use async_trait::async_trait;
use serde_json;

use kirkiano_util::{error::ebox, sr};
use crate::LogMessage;



/// Thin wrapper around [io::Stdout]
#[derive(Debug)]
pub struct Stdout(io::Stdout);


impl Stdout {
    pub fn new() -> Self {
        Self(io::stdout())
    }
}


#[async_trait]
impl sr::Sender<LogMessage> for Stdout {
    async fn send(&mut self, m: LogMessage) -> sr::Result<()> {
        serde_json::to_writer(&mut self.0, &m)
            .map_err(|ioerr| sr::Error::Other(ebox(ioerr)))?;
        self.0.flush().map_err(|ioerr| sr::Error::Other(ebox(ioerr)))?;
        Ok(())
    }
}