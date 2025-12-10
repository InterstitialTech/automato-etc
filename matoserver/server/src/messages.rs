use crate::serial_error;
use automato::automatomsg as am;
use elm_rs::{Elm, ElmJson};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ServerResponse {
    pub what: String,
    pub content: Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicMessage {
    pub what: String,
    pub data: Option<serde_json::Value>,
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
