use crate::serial_error;
use automato::automatomsg::{self as am, AutomatoId};
use elm_rs::{Elm, ElmJson};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Elm, ElmJson)]
pub enum ServerResponse {
    SrAutomatos(Vec<AutomatoId>),
    SrAutomatoMsg(AutomatoMsg),
    SrSerialPorts(Vec<SerialPortInfo>),
    SrSerialError(serial_error::Error),
    SrGenericError(String),
}

#[derive(Deserialize, Serialize, Debug, Elm, ElmJson)]
pub enum PublicMessage {
    PrGetAutomatoList,
    PrAutomatoMsg(AutomatoMsg),
    PrGetSerialPortList,
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

pub fn get_port_info(port: &serialport::SerialPortInfo) -> SerialPortInfo {
    SerialPortInfo {
        port_name: port.port_name.clone(),
        port_type: match &port.port_type {
            serialport::SerialPortType::UsbPort(info) => SerialPortType::UsbPort(UsbPortInfo {
                vid: info.vid,
                pid: info.pid,
                serial_number: info.serial_number.clone(),
                manufacturer: info.manufacturer.clone(),
                product: info.product.clone(),
            }),
            serialport::SerialPortType::BluetoothPort => SerialPortType::BluetoothPort,
            serialport::SerialPortType::PciPort => SerialPortType::PciPort,
            serialport::SerialPortType::Unknown => SerialPortType::Unknown,
        },
    }
}

// ------------------------------------------------
// copy of serialport lib structs, but with Elm, ElmJson

/// Contains all possible USB information about a `SerialPort`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Elm, ElmJson)]
pub struct UsbPortInfo {
    /// Vendor ID
    pub vid: u16,
    /// Product ID
    pub pid: u16,
    /// Serial number (arbitrary string)
    pub serial_number: Option<String>,
    /// Manufacturer (arbitrary string)
    pub manufacturer: Option<String>,
    /// Product name (arbitrary string)
    pub product: Option<String>,
}

/// The physical type of a `SerialPort`
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Elm, ElmJson)]
pub enum SerialPortType {
    /// The serial port is connected via USB
    UsbPort(UsbPortInfo),
    /// The serial port is connected via PCI (permanent port)
    PciPort,
    /// The serial port is connected via Bluetooth
    BluetoothPort,
    /// It can't be determined how the serial port is connected
    Unknown,
}

/// A device-independent implementation of serial port information
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Elm, ElmJson)]
pub struct SerialPortInfo {
    /// The short name of the serial port
    pub port_name: String,
    /// The hardware device type that exposes this port
    pub port_type: SerialPortType,
}
