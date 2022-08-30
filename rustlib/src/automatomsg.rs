use elm_rs::{Elm, ElmJson};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::de::Deserializer;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use serial;
use std::io::{Read, Write};
use std::mem::size_of;
// --------------------------------------------------------
// message structs.
// --------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum PayloadType {
    PtAck = 0,
    PtFail = 1,
    PtPinmode = 2,
    PtReadpin = 3,
    PtReadpinreply = 4,
    PtWritepin = 5,
    PtReadmem = 6,
    PtReadmemreply = 7,
    PtWritemem = 8,
    PtReadinfo = 9,
    PtReadinforeply = 10,
    PtReadhumidity = 11,
    PtReadhumidityreply = 12,
    PtReadtemperature = 13,
    PtReadtemperaturereply = 14,
    PtReadanalog = 15,
    PtReadanalogreply = 16,
    PtReadfield = 17,
    PtReadfieldreply = 18,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
pub enum FieldFormat {
    FfString = 0, // called ff_char on the C++ side
    FfFloat = 1,
    FfUint8 = 2,
    FfUint16 = 3,
    FfUint32 = 4,
    FfInt8 = 5,
    FfInt16 = 6,
    FfInt32 = 7,
    FfOther = 8,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
#[repr(C)]
#[repr(packed)]
pub struct RemoteInfo {
    pub protoversion: f32,
    pub mac_address: u64,
    pub datalen: u16,
    pub fieldcount: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
#[repr(C)]
#[repr(packed)]
pub struct Pinval {
    pub pin: u8,
    pub state: u8,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
#[repr(C)]
#[repr(packed)]
pub struct AnalogPinval {
    pub pin: u8,
    pub state: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
#[repr(C)]
#[repr(packed)]
pub struct Pinmode {
    pub pin: u8,
    pub mode: u8,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
#[repr(C)]
#[repr(packed)]
pub struct Readmem {
    pub address: u16,
    pub length: u8,
}

pub const RH_RF95_MAX_MESSAGE_LEN: usize = 251; // 255 - 4.

// #define MAX_WRITEMEM RH_RF95_MAX_MESSAGE_LEN - sizeof(u16) - sizeof(u8) - sizeof(u8)
const MAX_WRITEMEM: usize = 247;
// #define MAX_READMEM RH_RF95_MAX_MESSAGE_LEN - sizeof(u8) - sizeof(u8)
const MAX_READMEM: usize = 249;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct ReadmemReplyUnion {
    pub length: u8,
    pub data: [u8; MAX_READMEM],
}

#[derive(Clone, Debug, Elm, ElmJson, Serialize, Deserialize)]
pub struct ReadmemReply {
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
#[repr(packed)]
pub struct WritememUnion {
    pub address: u16,
    pub length: u8,
    pub data: [u8; MAX_WRITEMEM],
}

#[derive(Clone, Serialize, Deserialize, Debug, Elm, ElmJson)]
pub struct Writemem {
    pub address: u16,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
#[repr(C)]
#[repr(packed)]
pub struct ReadField {
    pub index: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Elm, ElmJson)]
#[repr(C)]
#[repr(packed)]
pub struct ReadFieldReply {
    pub index: u16,
    pub offset: u16,
    pub length: u8,
    pub format: u8,
    pub name: [u8; 25],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub union PayloadData {
    pub pinval: Pinval,
    pub pinmode: Pinmode,
    pub analogpinval: AnalogPinval,
    pub readmem: Readmem,
    pub readmemreply: ReadmemReplyUnion,
    pub writemem: WritememUnion,
    pub remoteinfo: RemoteInfo,
    pub readfield: ReadField,
    pub readfieldreply: ReadFieldReply,
    pub failcode: u8,
    pub pin: u8,
    pub f: f32,
    pub unit: (),
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Payload {
    pub payload_type: PayloadType,
    pub data: PayloadData,
}

#[derive(Clone, Serialize, Deserialize, Debug, Elm, ElmJson)]
pub enum PayloadEnum {
    PeAck,
    PeFail(u8),
    PePinmode(Pinmode),
    PeReadpin(u8),
    PeReadpinreply(Pinval),
    PeWritepin(Pinval),
    PeReadmem(Readmem),
    PeReadmemreply(ReadmemReply),
    PeWritemem(Writemem),
    PeReadinfo,
    PeReadinforeply(RemoteInfo),
    PeReadhumidity,
    PeReadhumidityreply(f32),
    PeReadtemperature,
    PeReadtemperaturereply(f32),
    PeReadanalog(u8),
    PeReadanalogreply(AnalogPinval),
    PeReadfield(ReadField),
    PeReadfieldreply(ReadFieldReply),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PayloadSerde {
    pub payload: PayloadEnum,
}

impl From<Payload> for PayloadEnum {
    fn from(payload: Payload) -> PayloadEnum {
        unsafe {
            match payload.payload_type {
                PayloadType::PtAck => PayloadEnum::PeAck,
                PayloadType::PtFail => PayloadEnum::PeFail(payload.data.failcode),
                PayloadType::PtPinmode => PayloadEnum::PePinmode(payload.data.pinmode),
                PayloadType::PtReadpin => PayloadEnum::PeReadpin(payload.data.pin),
                PayloadType::PtReadpinreply => PayloadEnum::PeReadpinreply(payload.data.pinval),
                PayloadType::PtWritepin => PayloadEnum::PeWritepin(payload.data.pinval),
                PayloadType::PtReadanalog => PayloadEnum::PeReadanalog(payload.data.pin),
                PayloadType::PtReadanalogreply => {
                    PayloadEnum::PeReadanalogreply(payload.data.analogpinval)
                }
                PayloadType::PtReadmem => PayloadEnum::PeReadmem(payload.data.readmem),
                PayloadType::PtReadmemreply => {
                    let rmr = ReadmemReply {
                        data: payload.data.readmemreply.data
                            [0..payload.data.readmemreply.length as usize]
                            .to_vec(),
                    };
                    PayloadEnum::PeReadmemreply(rmr)
                }
                PayloadType::PtWritemem => {
                    let wm = Writemem {
                        address: payload.data.writemem.address,
                        data: payload.data.writemem.data[0..payload.data.writemem.length as usize]
                            .to_vec(),
                    };
                    PayloadEnum::PeWritemem(wm)
                }
                // PayloadEnum::PeWritemem(payload.data.writemem),
                PayloadType::PtReadinfo => PayloadEnum::PeReadinfo,
                PayloadType::PtReadinforeply => {
                    PayloadEnum::PeReadinforeply(payload.data.remoteinfo)
                }
                PayloadType::PtReadhumidity => PayloadEnum::PeReadhumidity,
                PayloadType::PtReadhumidityreply => {
                    PayloadEnum::PeReadhumidityreply(payload.data.f)
                }
                PayloadType::PtReadtemperature => PayloadEnum::PeReadtemperature,
                PayloadType::PtReadtemperaturereply => {
                    PayloadEnum::PeReadtemperaturereply(payload.data.f)
                }
                PayloadType::PtReadfield => PayloadEnum::PeReadfield(payload.data.readfield),
                PayloadType::PtReadfieldreply => {
                    PayloadEnum::PeReadfieldreply(payload.data.readfieldreply)
                }
            }
        }
    }
}
impl From<PayloadEnum> for Payload {
    fn from(pe: PayloadEnum) -> Payload {
        let mut payload = Payload {
            payload_type: PayloadType::PtAck,
            data: PayloadData { unit: () },
        };
        match pe {
            PayloadEnum::PeAck => {
                payload.payload_type = PayloadType::PtAck;
            }
            PayloadEnum::PeFail(failcode) => {
                payload.payload_type = PayloadType::PtFail;
                payload.data.failcode = failcode
            }
            PayloadEnum::PePinmode(pinmode) => {
                payload.payload_type = PayloadType::PtPinmode;
                payload.data.pinmode = pinmode
            }
            PayloadEnum::PeReadpin(pin) => {
                payload.payload_type = PayloadType::PtReadpin;
                payload.data.pin = pin
            }
            PayloadEnum::PeReadpinreply(pinval) => {
                payload.payload_type = PayloadType::PtReadpinreply;
                payload.data.pinval = pinval
            }
            PayloadEnum::PeWritepin(pinval) => {
                payload.payload_type = PayloadType::PtWritepin;
                payload.data.pinval = pinval
            }
            PayloadEnum::PeReadanalog(pin) => {
                payload.payload_type = PayloadType::PtReadanalog;
                payload.data.pin = pin
            }
            PayloadEnum::PeReadanalogreply(analogpinval) => {
                payload.payload_type = PayloadType::PtReadanalogreply;
                payload.data.analogpinval = analogpinval
            }
            PayloadEnum::PeReadmem(readmem) => {
                payload.payload_type = PayloadType::PtReadmem;
                payload.data.readmem = readmem
            }
            PayloadEnum::PeReadmemreply(readmemreply) => {
                payload.payload_type = PayloadType::PtReadmemreply;
                let mut r = ReadmemReplyUnion {
                    length: readmemreply.data.len() as u8,
                    data: [0; MAX_READMEM],
                };
                // copy in data.
                let mut toidx = 0;
                for x in readmemreply.data {
                    r.data[toidx] = x;
                    toidx = toidx + 1;
                }
                payload.data.readmemreply = r
            }
            PayloadEnum::PeWritemem(writemem) => {
                payload.payload_type = PayloadType::PtWritemem;
                let mut w = WritememUnion {
                    address: writemem.address,
                    length: writemem.data.len() as u8,
                    data: [0; MAX_WRITEMEM],
                };
                // copy in data.
                let mut toidx = 0;
                for x in writemem.data {
                    w.data[toidx] = x;
                    toidx = toidx + 1;
                }
                payload.data.writemem = w
            }
            PayloadEnum::PeReadinfo => {
                payload.payload_type = PayloadType::PtReadinfo;
            }
            PayloadEnum::PeReadinforeply(remoteinfo) => {
                payload.payload_type = PayloadType::PtReadinforeply;
                payload.data.remoteinfo = remoteinfo
            }
            PayloadEnum::PeReadhumidity => {
                payload.payload_type = PayloadType::PtReadhumidity;
            }
            PayloadEnum::PeReadhumidityreply(f) => {
                payload.payload_type = PayloadType::PtReadhumidityreply;
                payload.data.f = f
            }
            PayloadEnum::PeReadtemperature => {
                payload.payload_type = PayloadType::PtReadtemperature;
            }
            PayloadEnum::PeReadtemperaturereply(f) => {
                payload.payload_type = PayloadType::PtReadtemperaturereply;
                payload.data.f = f
            }
            PayloadEnum::PeReadfield(readfield) => {
                payload.payload_type = PayloadType::PtReadfield;
                payload.data.readfield = readfield
            }
            PayloadEnum::PeReadfieldreply(readfieldreply) => {
                payload.payload_type = PayloadType::PtReadfieldreply;
                payload.data.readfieldreply = readfieldreply
            }
        }
        payload
    }
}

impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pe = PayloadEnum::from(self.clone());
        pe.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pe = PayloadEnum::deserialize(deserializer)?;

        Ok(Payload::from(pe))
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub union Msgbuf {
    pub buf: [u8; RH_RF95_MAX_MESSAGE_LEN],
    pub payload: Payload,
}

#[derive(Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive, Serialize, Deserialize, Debug)]
#[repr(u8)]
pub enum ResultCode {
    RcOk,
    RcNoMessageReceived,
    RcInvalidMessageType,
    RcInvalidPinNumber,
    RcInvalidMemAddress,
    RcInvalidMemLength,
    RcInvalidReplyMessage,
    RcOperationForbidden,
    RcReplyTimeout,
    RcRhRouterErrorInvalidLength,
    RcRhRouterErrorNoRoute,
    RcRhRouterErrorTimeout,
    RcRhRouterErrorNoReply,
    RcRhRouterErrorUnableToDeliver,
    RcInvalidRhRouterError,
    RcCount, // total number of ResultCodes.
}

// --------------------------------------------------------
// message fns.
// --------------------------------------------------------

pub fn payload_size(p: &Payload) -> usize {
    match p.payload_type {
        PayloadType::PtAck => size_of::<u8>(),
        PayloadType::PtFail => size_of::<u8>() + size_of::<u8>(),
        PayloadType::PtPinmode => size_of::<u8>() + size_of::<Pinmode>(),
        PayloadType::PtReadpin => size_of::<u8>() + size_of::<u8>(),
        PayloadType::PtReadpinreply => size_of::<u8>() + size_of::<Pinval>(),
        PayloadType::PtWritepin => size_of::<u8>() + size_of::<Pinval>(),
        PayloadType::PtReadanalog => size_of::<u8>() + size_of::<u8>(),
        PayloadType::PtReadanalogreply => size_of::<u8>() + size_of::<AnalogPinval>(),
        PayloadType::PtReadmem => size_of::<u8>() + size_of::<Readmem>(),
        PayloadType::PtReadmemreply => unsafe {
            size_of::<u8>() + size_of::<u8>() + p.data.readmemreply.length as usize
        },
        PayloadType::PtWritemem => unsafe {
            size_of::<u8>() + size_of::<u16>() + size_of::<u8>() + p.data.writemem.length as usize
        },
        PayloadType::PtReadinfo => size_of::<u8>(),
        PayloadType::PtReadinforeply => size_of::<u8>() + size_of::<RemoteInfo>(),
        PayloadType::PtReadhumidity => size_of::<u8>(),
        PayloadType::PtReadhumidityreply => size_of::<u8>() + size_of::<f32>(),
        PayloadType::PtReadtemperature => size_of::<u8>(),
        PayloadType::PtReadtemperaturereply => size_of::<u8>() + size_of::<f32>(),
        PayloadType::PtReadfield => size_of::<u8>() + size_of::<ReadField>(),
        PayloadType::PtReadfieldreply => size_of::<u8>() + size_of::<ReadFieldReply>(),
    }
}

pub fn setup_ack(p: &mut Payload) {
    p.payload_type = PayloadType::PtAck;
}

pub fn setup_fail(p: &mut Payload, rc: ResultCode) {
    p.payload_type = PayloadType::PtFail;
    p.data.failcode = rc as u8;
}

pub fn setup_pinmode(p: &mut Payload, pin: u8, mode: u8) {
    p.payload_type = PayloadType::PtPinmode;
    p.data.pinmode.pin = pin;
    p.data.pinmode.mode = mode;
}

pub fn setup_readpin(p: &mut Payload, pin: u8) {
    p.payload_type = PayloadType::PtReadpin;
    p.data.pin = pin;
}

pub fn setup_readpinreply(p: &mut Payload, pin: u8, state: u8) {
    p.payload_type = PayloadType::PtReadpinreply;
    p.data.pinval.pin = pin;
    p.data.pinval.state = state;
}

pub fn setup_writepin(p: &mut Payload, pin: u8, state: u8) {
    p.payload_type = PayloadType::PtWritepin;
    p.data.pinval.pin = pin;
    p.data.pinval.state = state;
}

pub fn setup_readanalog(p: &mut Payload, pin: u8) {
    p.payload_type = PayloadType::PtReadanalog;
    p.data.pin = pin;
}

pub fn setup_readanalogreply(p: &mut Payload, pin: u8, state: u16) {
    p.payload_type = PayloadType::PtReadanalogreply;
    p.data.analogpinval.pin = pin;
    p.data.analogpinval.state = state;
}

pub fn setup_readmem(p: &mut Payload, address: u16, length: u8) {
    p.payload_type = PayloadType::PtReadmem;
    p.data.readmem.address = address;
    p.data.readmem.length = length;
}

pub fn setup_readmemreply(p: &mut Payload, mem: &[u8]) -> ResultCode {
    p.payload_type = PayloadType::PtReadmemreply;
    if mem.len() <= MAX_READMEM {
        p.data.readmemreply.length = mem.len() as u8;
        unsafe {
            p.data.readmemreply.data[0..mem.len()].copy_from_slice(&mem);
        }
        ResultCode::RcOk
    } else {
        ResultCode::RcInvalidMemLength
    }
}

pub fn setup_writemem(p: &mut Payload, address: u16, mem: &[u8]) -> ResultCode {
    p.payload_type = PayloadType::PtWritemem;
    if mem.len() <= MAX_WRITEMEM {
        p.data.writemem.address = address;
        p.data.writemem.length = mem.len() as u8;
        unsafe {
            p.data.writemem.data[0..mem.len()].copy_from_slice(&mem);
        }
        ResultCode::RcOk
    } else {
        ResultCode::RcInvalidMemLength
    }
}

pub fn setup_readinfo(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadinfo;
}

pub fn setup_readinforeply(
    p: &mut Payload,
    protoversion: f32,
    mac_address: u64,
    datalen: u16,
    fieldcount: u16,
) {
    p.payload_type = PayloadType::PtReadinforeply;
    p.data.remoteinfo.protoversion = protoversion;
    p.data.remoteinfo.mac_address = mac_address;
    p.data.remoteinfo.datalen = datalen;
    p.data.remoteinfo.fieldcount = fieldcount;
}

pub fn setup_readfield(p: &mut Payload, index: u16) {
    p.payload_type = PayloadType::PtReadfield;
    p.data.readfield.index = index;
}

pub fn setup_readfieldreply(
    p: &mut Payload,
    index: u16,
    offset: u16,
    length: u8,
    format: FieldFormat,
    name: &[u8],
) {
    p.payload_type = PayloadType::PtReadfieldreply;
    p.data.readfieldreply.index = index;
    p.data.readfieldreply.offset = offset;
    p.data.readfieldreply.length = length;
    p.data.readfieldreply.format = format as u8;
    unsafe {
        p.data.readfieldreply.name[0..name.len()].copy_from_slice(&name);
    }
}

pub fn setup_readhumidity(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadhumidity;
}

pub fn setup_readhumidityreply(p: &mut Payload, humidity: f32) {
    p.payload_type = PayloadType::PtReadhumidityreply;
    p.data.f = humidity;
}

pub fn setup_readtemperature(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadtemperature;
}

pub fn setup_readtemperaturereply(p: &mut Payload, temperature: f32) {
    p.payload_type = PayloadType::PtReadtemperaturereply;
    p.data.f = temperature;
}

pub unsafe fn print_payload(p: &Payload) {
    println!("message payload");

    match p.payload_type {
        PayloadType::PtAck => {
            println!("PtAck");
        }
        PayloadType::PtFail => {
            println!("PtFail; ");
            println!("code: {}", { p.data.failcode });
            // println!(resultString((ResultCode)p.data.failcode));
        }
        PayloadType::PtPinmode => {
            println!("PtPinmode");
            println!("pin: {}", { p.data.pinmode.pin });
            println!("mode: {}", { p.data.pinmode.mode });
        }
        PayloadType::PtReadpin => {
            println!("PtReadpin");
            println!("pin: {}", { p.data.pin });
        }
        PayloadType::PtReadpinreply => {
            println!("PtReadpinreply");
            println!("pin: {}", { p.data.pinval.pin });
            println!("state: {}", { p.data.pinval.state });
        }
        PayloadType::PtWritepin => {
            println!("PtWritepin");
            println!("pin: {}", { p.data.pinval.pin });
            println!("state: {}", { p.data.pinval.state });
        }
        PayloadType::PtReadanalog => {
            println!("PtReadanalog");
            println!("pin: {}", { p.data.pin });
        }
        PayloadType::PtReadanalogreply => {
            println!("PtReadanalogreply");
            println!("pin: {}", { p.data.pin });
            println!("state: {}", { p.data.analogpinval.state });
        }
        PayloadType::PtReadmem => {
            println!("PtReadmem");
            println!("address{}", { p.data.readmem.address });
            println!("length{}", { p.data.readmem.length });
        }
        PayloadType::PtReadmemreply => {
            println!("PtReadmemreply");
            println!("length: {}", { p.data.readmemreply.length });
            println!("values");
            println!("offset: decimal, hex ");
            for i in 0..p.data.readmemreply.length as usize {
                let c = p.data.readmemreply.data[i];
                println!("{}: {}, {:#X}", i, c, c);
            }
        }
        PayloadType::PtWritemem => {
            println!("PtWritemem");
            println!("address {}", { p.data.writemem.address });
            println!("length {}", { p.data.writemem.length });
            println!("offset: decimal, hex ");
            for i in 0..p.data.writemem.length as usize {
                let c = p.data.writemem.data[i];
                println!("{}: {}, {:#X}", i, c, c);
            }
        }
        PayloadType::PtReadinfo => {
            println!("PtReadinfo");
        }
        PayloadType::PtReadinforeply => {
            println!("PtReadinforeply");
            println!("protoversion:{}", { p.data.remoteinfo.protoversion });
            println!("macAddress:{}", { p.data.remoteinfo.mac_address });
            println!("datalen:{}", { p.data.remoteinfo.datalen });
            println!("fieldcount:{}", { p.data.remoteinfo.fieldcount });
        }
        PayloadType::PtReadhumidity => {
            println!("PtReadhumidity");
        }
        PayloadType::PtReadhumidityreply => {
            println!("PtReadhumidityreply");
            println!("humidity:{}", { p.data.f });
        }
        PayloadType::PtReadtemperature => {
            println!("PtReadtemperature");
        }
        PayloadType::PtReadtemperaturereply => {
            println!("PtReadtemperaturereply");
            println!("temperature:{}", { p.data.f });
        }

        PayloadType::PtReadfield => {
            println!("PtReadfieldreply");
            println!("index: {}", { p.data.readfieldreply.index });
        }
        PayloadType::PtReadfieldreply => {
            println!("PtReadfieldreply");
            println!("index: {}", { p.data.readfieldreply.index });
            println!("offset: {}", { p.data.readfieldreply.offset });
            println!("length: {}", p.data.readfieldreply.length);
            println!("format: {:?}", p.data.readfieldreply.format);
            print!("name: ");
            for i in 0..p.data.readfieldreply.name.len() {
                print!("{}", p.data.readfieldreply.name[i] as char);
            }
            println!("");
        }
    }
}

pub unsafe fn write_message(
    port: &mut serial::SystemPort,
    msg: &Msgbuf,
    toid: u8,
) -> Result<(), serial::Error> {
    let sz = payload_size(&msg.payload);

    port.write(&['m' as u8])?;
    port.write(&[toid as u8])?;
    port.write(&[sz as u8])?;
    port.write(&msg.buf[0..sz + 1])?;

    Ok(())
}

pub unsafe fn read_message(
    port: &mut serial::SystemPort,
    msg: &mut Msgbuf,
    fromid: &mut u8,
) -> Result<bool, serial::Error> {
    let mut monobuf = [0; 1];

    port.read_exact(&mut monobuf)?;
    if monobuf[0] as char != 'm' {
        return Ok(false);
    }

    port.read_exact(&mut monobuf)?;
    *fromid = monobuf[0];

    port.read_exact(&mut monobuf)?;
    let sz = monobuf[0] as usize;

    if sz > 0 {
        port.read_exact(&mut msg.buf[0..sz])?;
    }

    Ok(true)
}
