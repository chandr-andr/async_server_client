mod handler;

use base::server::{SmartHouseConnection, SmartHouseServer};
use handler::{Request, RequestHandler};
use smart_device::SmartSocket;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{fs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = SmartHouseServer::bind(addr).await?;
    let socket = Arc::new(Mutex::new(SmartSocket::new("test")));

    loop {
        let connection = match server.incoming().await {
            Ok(conn) => conn,
            Err(_) => continue,
        };

        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);
        let socket = Arc::clone(&socket);
        tokio::spawn(async move { handle_connection(connection, socket).await });
    }
}

async fn handle_connection(
    mut connection: SmartHouseConnection,
    socket: Arc<Mutex<SmartSocket>>,
) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(socket);
    loop {
        let req_str = connection.recv_request().await?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req)).await?;
    }
}
