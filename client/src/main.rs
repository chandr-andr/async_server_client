mod state;

use base::client::{DeviceClient, RequestResult};
use base::error::ConnectResult;
use state::{Main, State};
use std::error::Error;
use std::fs;
use tokio::net::ToSocketAddrs;

pub struct SmartDeviceClient {
    stp: DeviceClient,
}

impl SmartDeviceClient {
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = DeviceClient::connect(addr).await?;
        Ok(Self { stp })
    }

    pub async fn toggle(&mut self) -> RequestResult {
        self.stp.send_request("toggle|||socket").await
    }

    pub async fn describe(&mut self) -> RequestResult {
        self.stp.send_request("describe|||socket").await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server_addr();
    let mut client = SmartDeviceClient::new(addr).await?;

    let mut state: Box<dyn State> = Box::new(Main);
    while !state.exit() {
        state = state.update(&mut client).await?;
    }

    Ok(())
}

fn get_server_addr() -> String {
    fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
