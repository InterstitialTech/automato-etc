use crate::data::{SerialPortInfo, SerialPortType};
use crate::serial_error;
use automato::automatomsg::{self as am, AutomatoId};
use elm_rs::{Elm, ElmJson};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Elm, ElmJson)]
pub enum ServerResponse {
    Automatos(Vec<AutomatoId>),
    AutomatoMsg(AutomatoMsg),
    SerialPorts(Vec<SerialPortInfo>),
    SerialError(serial_error::Error),
    GenericError(String),
}

#[derive(Deserialize, Serialize, Debug, Elm, ElmJson)]
pub enum PublicMessage {
    GetAutomatoList,
    AutomatoMsg(AutomatoMsg),
    GetSerialPortList,
}

#[derive(Serialize, Deserialize, Debug, Clone, Elm, ElmJson)]
pub struct AutomatoMsg {
    pub id: am::AutomatoId,
    pub message: am::PayloadEnum,
}

#[derive(Serialize, Deserialize, Debug, Clone, Elm, ElmJson)]
pub struct WhatMsg {
    pub what: String,
    pub msg: AutomatoMsg,
}

#[derive(Serialize, Deserialize, Debug, Clone, Elm, ElmJson)]
pub struct WhatError {
    pub what: String,
    pub msg: serial_error::Error,
}
