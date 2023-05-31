use std::{fmt, str::FromStr};

use kirkiano_util::{initia::New,
                    time::Timeout,
                    channel, idisplay::{self, *},
                    net::{listen, socket, Port, MaxLineLength},
                    settings::{clargs, get_env_var}};

use crate::settings::{Mongo, Result, Error};


/// General settings
#[derive(Debug)]
pub struct Settings {
    pub listen_params: listen::Params,
    pub ccap: channel::Bound,
    pub mongo: Mongo,
}


impl IDisplay for Settings {
    fn idisplay(&self, i: usize) -> String {
        let ind = idisplay::indent(i+1);
        format!("SETTINGS:\n{}\n{}\n{}",
                format!("{}Listener:\n{}", ind, self.listen_params.idisplay(i+2)),
                format!("{}Channel {}", ind, self.ccap),
                format!("{}{}", ind, self.mongo.idisplay(i+1)))
    }
}


impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SETTINGS:")?;
        writeln!(f, "{}", self.listen_params)?;
        writeln!(f, "Channel {}", self.ccap)?;
        writeln!(f, "{}", self.mongo)
    }
}


impl Settings {
    pub async fn get() -> Result<Settings> {
        let mongo = Mongo::get().await?;
        let ccap = get_channel_capacity()?;
        let listen_params = get_listen_params()?;
        let s = Settings { listen_params, mongo, ccap };
        Ok(s)
    }
}


fn get_listen_params() -> Result<listen::Params> {
    let args = clargs();
    let listen_locally = !args.is_empty() && args[0] == "--listen-locally";
    let port_s = get_env_var("PORT").map_err(|_| Error::NoPort)?;
    let port = Port::from_str(port_s.as_str())
                     .map_err(|e| Error::NotAPort(port_s, e))?;
    let address = listen::Address::new((listen_locally.into(), port));
    let mll_from: MaxLineLength = None.into();
    let timeout = Timeout::from(None);
    let constraints = socket::Constraints { mll_from, timeout }; 
    let listen_params = listen::Params { address, constraints };
    Ok(listen_params)
}


fn get_channel_capacity() -> Result<channel::Bound> {
    let ccap_s = get_env_var("CHANNEL_CAPACITY").unwrap_or("100".into());
    let ccap = usize::from_str(ccap_s.as_str())
                .map_err(|_| Error::NotAChannelCapacity(ccap_s))?;
    Ok(ccap.into())
}