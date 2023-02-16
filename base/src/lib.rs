pub mod client;
pub mod error;
pub mod server;

use crate::error::{RecvError, RecvResult, SendResult};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn send_string<Data: AsRef<str>>(data: Data, tcp_stream: &mut TcpStream) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    tcp_stream.write_all(&len_bytes).await?;
    tcp_stream.write_all(bytes).await?;
    Ok(())
}

async fn recv_string(tcp_stream: &mut TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    tcp_stream.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    tcp_stream.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}
