use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::mem::size_of;
// --------------------------------------------------------
// message structs.
// --------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
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

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct RemoteInfo {
    pub protoversion: f32,
    pub macAddress: u64,
    pub datalen: u16,
    pub fieldcount: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Pinval {
    pub pin: u8,
    pub state: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct AnalogPinval {
    pub pin: u8,
    pub state: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Pinmode {
    pub pin: u8,
    pub mode: u8,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct ReadmemReply {
    pub length: u8,
    pub data: [u8; MAX_READMEM],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Writemem {
    pub address: u16,
    pub length: u8,
    pub data: [u8; MAX_WRITEMEM],
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct ReadField {
    pub index: u16,
}

#[derive(Clone, Copy)]
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
    pub readmemreply: ReadmemReply,
    pub writemem: Writemem,
    pub remoteinfo: RemoteInfo,
    pub readfield: ReadField,
    pub readfieldreply: ReadFieldReply,
    pub failcode: u8,
    pub pin: u8,
    pub f: f32,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub struct Payload {
    pub payload_type: u8,
    pub data: PayloadData,
}

// used for non-mesh, non-routed comms.
#[repr(C)]
#[repr(packed)]
pub struct Message {
    pub fromid: u8,
    pub toid: u8,
    pub data: Payload,
}

#[derive(Clone, Copy)]
#[repr(C)]
#[repr(packed)]
pub union Msgbuf {
    pub buf: [u8; RH_RF95_MAX_MESSAGE_LEN],
    pub payload: Payload,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, FromPrimitive, ToPrimitive)]
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

pub fn payloadSize(p: &Payload) -> usize {
    match PayloadType::from_u8(p.payload_type) {
        Some(pt) => match pt {
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
                size_of::<u8>()
                    + size_of::<u16>()
                    + size_of::<u8>()
                    + p.data.writemem.length as usize
            },
            PayloadType::PtReadinfo => size_of::<u8>(),
            PayloadType::PtReadinforeply => size_of::<u8>() + size_of::<RemoteInfo>(),
            PayloadType::PtReadhumidity => size_of::<u8>(),
            PayloadType::PtReadhumidityreply => size_of::<u8>() + size_of::<f32>(),
            PayloadType::PtReadtemperature => size_of::<u8>(),
            PayloadType::PtReadtemperaturereply => size_of::<u8>() + size_of::<f32>(),
            PayloadType::PtReadfield => size_of::<u8>() + size_of::<ReadField>(),
            PayloadType::PtReadfieldreply => size_of::<u8>() + size_of::<ReadFieldReply>(),
        },
        None => 0,
    }
}

pub fn setup_ack(p: &mut Payload) {
    p.payload_type = PayloadType::PtAck as u8;
}

pub fn setup_fail(p: &mut Payload, rc: ResultCode) {
    p.payload_type = PayloadType::PtFail as u8;
    p.data.failcode = rc as u8;
}

pub fn setup_pinmode(p: &mut Payload, pin: u8, mode: u8) {
    p.payload_type = PayloadType::PtPinmode as u8;
    p.data.pinmode.pin = pin;
    p.data.pinmode.mode = mode;
}

pub fn setup_readpin(p: &mut Payload, pin: u8) {
    p.payload_type = PayloadType::PtReadpin as u8;
    p.data.pin = pin;
}

pub fn setup_readpinreply(p: &mut Payload, pin: u8, state: u8) {
    p.payload_type = PayloadType::PtReadpinreply as u8;
    p.data.pinval.pin = pin;
    p.data.pinval.state = state;
}

pub fn setup_writepin(p: &mut Payload, pin: u8, state: u8) {
    p.payload_type = PayloadType::PtWritepin as u8;
    p.data.pinval.pin = pin;
    p.data.pinval.state = state;
}

pub fn setup_readanalog(p: &mut Payload, pin: u8) {
    p.payload_type = PayloadType::PtReadanalog as u8;
    p.data.pin = pin;
}

pub fn setup_readanalogreply(p: &mut Payload, pin: u8, state: u16) {
    p.payload_type = PayloadType::PtReadanalogreply as u8;
    p.data.analogpinval.pin = pin;
    p.data.analogpinval.state = state;
}

pub fn setup_readmem(p: &mut Payload, address: u16, length: u8) {
    p.payload_type = PayloadType::PtReadmem as u8;
    p.data.readmem.address = address;
    p.data.readmem.length = length;
}

pub fn setup_readmemreply(p: &mut Payload, mem: &[u8]) -> ResultCode {
    p.payload_type = PayloadType::PtReadmemreply as u8;
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
    p.payload_type = PayloadType::PtWritemem as u8;
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
    p.payload_type = PayloadType::PtReadinfo as u8;
}

pub fn setup_readinforeply(
    p: &mut Payload,
    protoversion: f32,
    macAddress: u64,
    datalen: u16,
    fieldcount: u16,
) {
    p.payload_type = PayloadType::PtReadinforeply as u8;
    p.data.remoteinfo.protoversion = protoversion;
    p.data.remoteinfo.macAddress = macAddress;
    p.data.remoteinfo.datalen = datalen;
    p.data.remoteinfo.fieldcount = fieldcount;
}

pub fn setup_readfield(p: &mut Payload, index: u16) {
    p.payload_type = PayloadType::PtReadfield as u8;
    p.data.readfield.index = index;
}

pub fn setup_readfieldreply(p: &mut Payload, index: u16, offset: u16, length: u8, name: &[u8]) {
    p.payload_type = PayloadType::PtReadfieldreply as u8;
    p.data.readfieldreply.index = index;
    p.data.readfieldreply.offset = offset;
    p.data.readfieldreply.length = length;
    unsafe {
        p.data.readfieldreply.name[0..name.len()].copy_from_slice(&name);
    }
}

pub fn setup_readhumidity(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadhumidity as u8;
}

pub fn setup_readhumidityreply(p: &mut Payload, humidity: f32) {
    p.payload_type = PayloadType::PtReadhumidityreply as u8;
    p.data.f = humidity;
}

pub fn setup_readtemperature(p: &mut Payload) {
    p.payload_type = PayloadType::PtReadtemperature as u8;
}

pub fn setup_readtemperaturereply(p: &mut Payload, temperature: f32) {
    p.payload_type = PayloadType::PtReadtemperaturereply as u8;
    p.data.f = temperature;
}

pub unsafe fn print_payload(p: &Payload) {
    println!("message payload");

    match PayloadType::from_u8(p.payload_type) {
        None => println!("invalid type: {}", p.payload_type),

        Some(pt) => {
            match pt {
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
                    println!("macAddress:{}", { p.data.remoteinfo.macAddress });
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
                    println!("index: {}", p.data.readfieldreply.index);
                }
                PayloadType::PtReadfieldreply => {
                    println!("PtReadfieldreply");
                    println!("index: {}", p.data.readfieldreply.index);
                    println!("offset: {}", p.data.readfieldreply.offset);
                    println!("length: {}", p.data.readfieldreply.length);
                    println!("format: {}", p.data.readfieldreply.format);
                    print!("name: ");
                    for i in 0..p.data.readfieldreply.name.len() {
                        print!("{}", p.data.readfieldreply.name[i] as char);
                    }
                    println!("");
                }
            }
        }
    }
}
