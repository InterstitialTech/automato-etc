use crate::config::Config;
use serialport;
use std::sync::{Arc, Mutex};

pub struct ServerData {
    pub config: Config,
    pub port: Arc<Mutex<Option<Box<dyn serialport::SerialPort>>>>,
}
