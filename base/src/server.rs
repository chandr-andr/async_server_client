use crate::error::{ConnectError, ConnectResult, RecvResult, SendResult};
use std::io;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Represent STP server, that can accept incoming connections.
pub struct SmartHouseServer {
    tcp: TcpListener,
}

impl SmartHouseServer {
    /// Binds server to specefied socket.
    pub async fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(Self { tcp })
    }

    /// Blocking iterator for incoming connections.
    pub async fn incoming(&self) -> ConnectResult<SmartHouseConnection> {
        // self.tcp.incoming().map(|s| match s {
        //     Ok(s) => Self::try_handshake(s),
        //     Err(e) => Err(ConnectError::Io(e)),
        // })
        match self.tcp.accept().await {
            Ok((tcp_stream, _)) => Self::try_handshake(tcp_stream).await,
            Err(e) => Err(ConnectError::Io(e)),
        }
    }

    async fn try_handshake(mut stream: TcpStream) -> ConnectResult<SmartHouseConnection> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"serv").await?;
        Ok(SmartHouseConnection { stream })
    }
}

pub type BindResult = Result<SmartHouseServer, BindError>;

/// Bind to socket error
#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Represent connection from client.
///
/// Allows to receive requests and send responses.
pub struct SmartHouseConnection {
    stream: TcpStream,
}

impl SmartHouseConnection {
    /// Send response to client
    pub async fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_string(response, &mut self.stream).await
    }

    /// Receive requests from client
    pub async fn recv_request(&mut self) -> RecvResult {
        crate::recv_string(&mut self.stream).await
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
