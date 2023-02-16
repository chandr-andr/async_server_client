pub trait SmartDevice {
    fn toggle(&mut self) -> bool;
    fn describe(&self) -> String;
}

pub struct SmartSocket {
    name: String,
    state: bool,
    power: f64,
}

impl SmartDevice for SmartSocket {
    fn toggle(&mut self) -> bool {
        self.state = !self.state;
        self.state
    }

    fn describe(&self) -> String {
        match self.state {
            true => {
                format!("Socket {} works and using {} power", self.name, self.power,)
            }
            false => {
                format!(
                    "Socket {} not working and using {} power",
                    self.name, self.power,
                )
            }
        }
    }
}

impl SmartSocket {
    pub fn new(socket_name: &str) -> Self {
        Self {
            name: socket_name.to_string(),
            state: false,
            power: 20.0,
        }
    }
}
