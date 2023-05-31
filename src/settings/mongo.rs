use std::fmt;
use mongodb::options::ClientOptions;

use kirkiano_util::{idisplay::{self, *}, settings::get_env_var};
use crate::settings::{Result, Error};


/// Mongo settings
#[derive(Debug)]
pub struct Mongo {
    pub options: ClientOptions,
    pub db: String,
    pub collection: String,
}


impl Mongo {
    pub async fn get() -> Result<Self> {
        let option_s = get_env_var("MONGO_URI").map_err(|_| Error::NoMongoURI)?;
        let mut options = ClientOptions::parse_async(option_s.clone()).await
                            .map_err(|e| Error::InvalidMongoURI(option_s, e))?;

        options.app_name = Some("rpg".to_string());

        let db = get_env_var("MONGO_DB").unwrap_or("rpg".into());

        let collection = get_env_var("MONGO_COLLECTION")
                            .unwrap_or("logs".into());
        let m = Mongo { options, db, collection };
        Ok(m)
    }
}


impl IDisplay for Mongo {
    fn idisplay(&self, i: usize) -> String {
        let ind = idisplay::indent(i+1);
        format!("Mongo settings:\n{}\n{}\n{}",
                format!("{}URI: {:?}", ind, self.options.hosts),
                format!("{}Db: {}", ind, self.db),
                format!("{}Collection: {}", ind, self.collection))
    }
}


impl fmt::Display for Mongo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mongo settings:")?;
        writeln!(f, "Client options: {:?}", self.options.hosts)?;
        writeln!(f, "Db: {}", self.db)?;
        writeln!(f, "Collection: {}", self.collection)
    }
}
