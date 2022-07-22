use crate::Config;
use automato::automatomsg as am;
// use elm_rs::{Elm, ElmJson};
use elm_rs::{Elm, ElmJson};
use serde_derive::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// -------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAutomato {
    pub id: i64,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct AutomatoInfo {
//     pub id: i64,
//     pub info: am::RemoteInfo,
// }

#[derive(Serialize, Deserialize, Debug, Clone, Elm, ElmJson)]
pub struct AutomatoMsg {
    pub id: u8,
    pub message: am::PayloadEnum,
}

pub struct ServerData {
    pub config: Config,
    pub port: Arc<Mutex<serial::SystemPort>>,
}
