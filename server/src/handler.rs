use smart_device::{SmartDevice, SmartSocket};

use std::{
    str::Split,
    sync::{Arc, Mutex},
};

pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub struct RequestHandler {
    socket: Arc<Mutex<SmartSocket>>,
}

impl RequestHandler {
    pub fn new(socket: Arc<Mutex<SmartSocket>>) -> Self {
        Self { socket }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.next();
        match command {
            "toggle" => self.toggle(request),
            "describe" => self.describe(request),
            _ => "Bad command".into(),
        }
    }

    fn toggle(&self, _request: Request) -> String {
        let mut mut_socket = self.socket.lock().unwrap();
        mut_socket.toggle();

        "Socket changed state".into()
    }

    fn describe(&mut self, _request: Request) -> String {
        let mut_socket = self.socket.lock().unwrap();
        mut_socket.describe()
    }
}
