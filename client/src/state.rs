use std::io;

use anyhow::Ok;

use crate::SmartDeviceClient;
use async_trait::async_trait;

#[async_trait]
pub trait State {
    async fn update(
        &mut self,
        client: &mut SmartDeviceClient,
    ) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct Main;

#[async_trait]
impl State for Main {
    async fn update(
        &mut self,
        device: &mut SmartDeviceClient,
    ) -> Result<Box<dyn State>, anyhow::Error> {
        println!(
            "Select option:
    1) Change socket state
    2) Get socket description
    Other) Exit"
        );
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let selected = buf.trim();
        println!("Selected: {}", selected);

        match selected {
            "1" => {
                println!("{}", device.toggle().await?);
                Ok(Box::new(Self))
            }
            "2" => {
                println!("{}", device.describe().await?);
                Ok(Box::new(Self))
            }
            _ => Ok(Box::new(Exit)),
        }
    }
}

struct Exit;

#[async_trait]
impl State for Exit {
    async fn update(&mut self, _: &mut SmartDeviceClient) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}
