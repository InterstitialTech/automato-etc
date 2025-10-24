use crate::Config;
use serialport;
use std::sync::{Arc, Mutex};

pub struct ServerData {
    pub config: Config,
    pub port: Arc<Mutex<Box<serialport::SerialPort>>>,
}

