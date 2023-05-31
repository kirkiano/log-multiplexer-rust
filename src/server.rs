use tracing::{info, trace, error};

use kirkiano_util::{scrutare::*,
                    sr::{Sender, Receiver, Close, WaitUntilClosed}};
use crate::{Result, Error, LogMessage};


/// Server loop
///
/// For each connection arriving on `lsnr`, spawns a task that
/// puts the client's messages on the given channel `tx`.
pub async fn run<L, T, F, D>(mut lsnr: L, tx: D) -> Result<()>
where L: Receiver<(T, F)>,
      D: 'static + Send + Sender<LogMessage> + Clone + WaitUntilClosed,
      T: 'static + Send,
      F: 'static + Send + Receiver<LogMessage> + Close,
{
    info!(target: "server", "Starting up");
    loop {
        trace!(target: "server", "Waiting for next client");
        match lsnr.recv().await {
            Ok(c) => {
                info!(target: "server", "New client!");
                let tx2 = tx.clone();
                tokio::spawn(async move { handle(c, tx2).await; });
            },
            Err(_) => {
                return Err(Error::ListenFailure)
            }
        }
    }

}


// Client loop, conveys messages from client `c` to the channel `tx`.
async fn handle<D, T, F>(c: (T, F), tx: D)
where D: 'static + Send + Sender<LogMessage> + WaitUntilClosed,
      F: 'static + Send + Receiver<LogMessage> + Close,
{
    let (_to, from) = c;
    trace!(target: "server", "Client task spawned");
    let pred = |_: &LogMessage| false;
    let jh = from.spawn_relay_forever_to(tx, Some(pred)).await;
    let _ = jh.await
      .scrut_err(|e| error!(target: "server",
                            "Client bailed with error: {}", e));
}


#[cfg(test)]
mod test {
    use serde_json::Value;

    use kirkiano_util::{error::ResultEB,
                        initia::New,
                        Split,
                        channel::{self, Channel, Bound, Duplex},
                        sr::{Sender, Receiver}};
    use crate::{server::run, LogMessage};


    fn mock_listener() -> Channel<(channel::To<()>,
                                   channel::From<LogMessage>)>
    {
        Channel::new(Bound::from(None))
    }

    fn mock_client() -> (Duplex<LogMessage, ()>,
                         Duplex<(), LogMessage>)
    {
        Duplex::pair(Bound::from(None), Bound::from(None))
    }


    #[tokio::test]
    async fn test_server_loop() -> ResultEB<()> {
        let (mut lsn_snd, lsn_rcv) = mock_listener().split();
        let (tx, mut rx) = Channel::new(Bound::from(None)).split();
        tokio::spawn(async move { let _ = run(lsn_rcv, tx).await; });

        let (c1_distal, c1_proximal) = mock_client();
        let (c2_distal, c2_proximal) = mock_client();
        let msg1 = LogMessage::from(Value::Null);
        let msg2 = LogMessage::from(Value::Number(42.into()));

        lsn_snd.send(c1_proximal.split()).await?;
        lsn_snd.send(c2_proximal.split()).await?;

        let (mut c1_to, _) = c1_distal.split();
        c1_to.send(msg1.clone()).await?;

        let (mut c2_to, _) = c2_distal.split();
        c2_to.send(msg2.clone()).await?;

        let mut output = Vec::<LogMessage>::new();
        output.push(rx.recv().await?);
        output.push(rx.recv().await?);

        // serde_json::Value is neither Hash nor Ord, so can't
        // use a set. Must use vectors instead.
        let output_exp_1 = vec![msg1.clone(), msg2.clone()];
        let output_exp_2 = vec![msg2, msg1];

        assert!(output == output_exp_1 || output == output_exp_2);

        Ok(())
    }
}