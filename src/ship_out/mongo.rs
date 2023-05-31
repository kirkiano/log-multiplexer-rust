use async_trait::async_trait;
use mongodb::{Database, Client, Collection, bson::{self, Document}};

use kirkiano_util::{sr, initia::Construct, error::ebox};
use crate::{LogMessage, settings, Result, Error};


/// Connection to Mongo
#[derive(Debug)]
pub struct Mongo {
    _client: Client,
    _db: Database,
    collection: Collection<Document>,
}


#[async_trait]
impl Construct<settings::Mongo, Result<Self>> for Mongo {
    async fn construct(s: settings::Mongo) -> Result<Mongo> {
        let _client = Client::with_options(s.options)
                         .map_err(Error::CannotConnectToMongo)?;
        let _db = _client.database(s.db.as_str());
        let collection = _db.collection::<Document>(s.collection.as_str());
        let m = Self { _client, _db, collection };
        Ok(m)
    }
}



#[async_trait]
impl sr::Sender<LogMessage> for Mongo {
    async fn send(&mut self, m: LogMessage) -> sr::Result<()> {
        // See https://stackoverflow.com/a/68016107
        let value = serde_json::Value::from(m);
        let doc = bson::to_document(&value)
            .map_err(|e| sr::Error::Other(ebox(e)))?;
        self.collection.insert_one(doc, None).await
            .map_err(|e| sr::Error::Other(ebox(e)))?;
        Ok(())
    }
}