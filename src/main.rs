use tokio::task::JoinHandle;

use kirkiano_util::{initia::{New, Construct},
                    sr::Sender, logging, scrutare::*,
                    channel, Split, idisplay::*,
                    net::listen};
use log_multiplexer::{server, Mongo, Result, Error,
                      Listener, LogMessage, Settings};


#[tokio::main]
async fn main() -> Result<()> {
    let _ = logging::init().scrut_err(|e| println!("Log cannot init: {}", e));

    let ss = Settings::get().await.map_err(Error::Settings)?;
    println!("\n{}\n", ss.idisplay(0));

    let mongo = Mongo::construct(ss.mongo).await?;

    let (rx, jh_server) = run_server(ss.listen_params, ss.ccap).await;
    mongo.spawn_relay_forever_from(rx).await;
    let _ = jh_server.await;

    Ok(())
}


/*
Spawn a task that listens for incoming streams and multiplex them
onto the returned channel.
*/
async fn run_server(lp: listen::Params,
                    cap: channel::Bound) -> (channel::From<LogMessage>,
                                             JoinHandle<Result<()>>)
{
    let lsnr: Listener = Construct::<listen::Params>::construct(lp).await;
    let (tx, rx) = channel::Channel::new(cap).split();
    let jh = tokio::spawn(async move { server::run(lsnr, tx).await });
    (rx, jh)
}