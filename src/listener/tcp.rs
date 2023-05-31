use std::net::SocketAddr;
use tracing::trace;
use async_trait::async_trait;
use tokio::net::TcpListener;

use kirkiano_util::{initia::{New, Construct},
                    sr, Split, serde::SerDe, scrutare::*,
                    net::{self, tcp::{self, Conn}, listen}};
use crate::Error;


pub struct Listener {
    params: listen::Params,
    inner: TcpListener,
}


#[async_trait]
impl Construct<net::listen::Params> for Listener {
    async fn construct(params: listen::Params) -> Self {
        let sa: SocketAddr = params.address.into();
        TcpListener::bind(sa).await
            .map(|inner| Listener { params, inner })
            .map_err(Error::CannotBind)
            .unwrap()
    }
}


type Client = (SerDe<tcp::To>,
               SerDe<tcp::From>);

#[async_trait]
impl sr::Receiver<Client> for Listener {
    async fn recv(&mut self) -> sr::Result<Client> {
        self.inner
            .accept().await
            .map_err(sr::Error::other)
            .scrut(|_| trace!(target: "tcp", "Incoming socket!"))
            .map(|(sock, _)| Conn::new((sock, self.params.constraints)))
            .map(|c| c.split())
            .map(|(t, f)| (SerDe::from(t), SerDe::from(f)))
    }
}