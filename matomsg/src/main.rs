use automato::automatomsg as am;
use automato::automatomsg::{
    AnalogPinval, Message, Msgbuf, Payload, PayloadData, PayloadType, Pinmode, Pinval, Readmem,
    ReadmemReply, RemoteInfo, ResultCode, Writemem,
};
use clap::{Arg, Command, SubCommand};
use num_derive;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use simple_error::bail;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::mem::size_of;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use serial::{BaudRate, CharSize, FlowControl, Parity, PortSettings, SerialPort, StopBits};

fn main() {
    match err_main() {
        Ok(()) => (),
        Err(e) => {
            println!("error: {:?}", e)
        }
    }
}

fn err_main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("matomsg")
        .version("1.0")
        .author("Automato Enterprises")
        .about("cli for testing automato messsages over serial.")
        .arg(
            Arg::with_name("port")
                .short('p')
                .long("port")
                .value_name("FILE")
                .help("serial port")
                .default_value("/dev/ttyUSB0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("baud")
                .short('b')
                .long("baud")
                .value_name("NUMBER")
                .help(
                    "baud rate: 110, 300, 600, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200",
                )
                .default_value("115200")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("address")
                .short('a')
                .long("address")
                .value_name("0-255")
                .help("lora network address of an automato")
                .required(true)
                .takes_value(true),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("writepin")
                .about("write 0 or 1 to pin")
                .arg(Arg::with_name("pin").value_name("PIN").takes_value(true))
                .arg(
                    Arg::with_name("value")
                        .value_name("1 or 0")
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("pinmode")
                .about("write 0 or 1 to pin")
                .arg(Arg::with_name("pin").value_name("PIN").takes_value(true))
                .arg(
                    Arg::with_name("value")
                        .value_name("1 or 0")
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("readpin")
                .about("query pin")
                .arg(Arg::with_name("pin").value_name("PIN").takes_value(true)),
        )
        .subcommand(
            Command::new("readanalog")
                .about("query pin")
                .arg(Arg::with_name("pin").value_name("PIN").takes_value(true)),
        )
        .subcommand(Command::new("readinfo").about("read automato general info"))
        .subcommand(Command::new("readhumidity").about("read automato humidity"))
        .subcommand(Command::new("readtemperature").about("read automato temperature"))
        .subcommand(
            Command::new("writemem")
                .about("write hex data to automato memory")
                .arg(
                    Arg::with_name("address")
                        .value_name("NUMBER")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("value")
                        .value_name("hex string")
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("readmem")
                .about("read hex data from automato memory")
                .arg(
                    Arg::with_name("address")
                        .value_name("NUMBER")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("length")
                        .value_name("NUMBER")
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("readfield")
                .about("read field info from automato memory map")
                .arg(
                    Arg::with_name("index")
                        .value_name("NUMBER")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let (port, baud, automatoaddr) = match (
        matches.value_of("port"),
        matches.value_of("baud"),
        matches.value_of("address"),
    ) {
        (Some(port), Some(baudstr), Some(addrstr)) => {
            let baud = BaudRate::from_speed(baudstr.parse::<usize>()?);
            let addr = addrstr.parse::<u8>()?;
            (port, baud, addr)
        }
        _ => bail!("arg failure"),
    };

    let mut mb = Msgbuf {
        buf: [0; am::RH_RF95_MAX_MESSAGE_LEN],
    };

    let mut retmsg = mb.clone();

    // set up the outgoing message.
    match matches.subcommand() {
        Some(("writepin", sub_matches)) => {
            let (pin, val) = match (sub_matches.value_of("pin"), sub_matches.value_of("value")) {
                (Some(pinstr), Some(valstr)) => (pinstr.parse::<u8>()?, valstr.parse::<u8>()?),
                _ => bail!("arg failure"),
            };

            unsafe { am::setup_writepin(&mut mb.payload, pin, val) };
        }
        Some(("pinmode", sub_matches)) => {
            let (pin, val) = match (sub_matches.value_of("pin"), sub_matches.value_of("value")) {
                (Some(pinstr), Some(valstr)) => (pinstr.parse::<u8>()?, valstr.parse::<u8>()?),
                _ => bail!("arg failure"),
            };

            unsafe { am::setup_pinmode(&mut mb.payload, pin, val) };
        }
        Some(("readpin", sub_matches)) => {
            let pin = match sub_matches.value_of("pin") {
                Some(pinstr) => pinstr.parse::<u8>()?,
                _ => bail!("arg failure"),
            };
            unsafe { am::setup_readpin(&mut mb.payload, pin) };
        }
        Some(("readanalog", sub_matches)) => {
            let pin = match sub_matches.value_of("pin") {
                Some(pinstr) => pinstr.parse::<u8>()?,
                _ => bail!("arg failure"),
            };
            unsafe { am::setup_readanalog(&mut mb.payload, pin) };
        }
        Some(("readinfo", sub_matches)) => {
            unsafe { am::setup_readinfo(&mut mb.payload) };
        }
        Some(("readhumidity", sub_matches)) => {
            unsafe { am::setup_readhumidity(&mut mb.payload) };
        }
        Some(("readtemperature", sub_matches)) => {
            unsafe { am::setup_readtemperature(&mut mb.payload) };
        }
        Some(("writemem", sub_matches)) => {
            let (addr, val) = match (
                sub_matches.value_of("address"),
                sub_matches.value_of("value"),
            ) {
                (Some(addrstr), Some(valstr)) => (addrstr.parse::<u16>()?, hex::decode(valstr)?),
                _ => bail!("arg failure"),
            };

            unsafe { am::setup_writemem(&mut mb.payload, addr, val.as_slice()) };
        }
        Some(("readmem", sub_matches)) => {
            let (addr, len) = match (
                sub_matches.value_of("address"),
                sub_matches.value_of("length"),
            ) {
                (Some(addrstr), Some(lenstr)) => (addrstr.parse::<u16>()?, lenstr.parse::<u8>()?),
                _ => bail!("arg failure"),
            };
            unsafe { am::setup_readmem(&mut mb.payload, addr, len) };
        }
        Some(("readfield", sub_matches)) => {
            let index = match sub_matches.value_of("index") {
                Some(istr) => istr.parse::<u16>()?,
                _ => bail!("arg failure"),
            };
            unsafe { am::setup_readfield(&mut mb.payload, index) };
        }
        meh => {
            bail!("unhandled command! {:?}", meh)
        }
    }

    let mut port = serial::open(port)?;

    let ps = PortSettings {
        baud_rate: baud,
        char_size: CharSize::Bits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::Stop1,
        flow_control: FlowControl::FlowNone,
    };
    port.configure(&ps)?;

    let readReply = true;
    unsafe {
        writeMessage(&mut port, &mb, automatoaddr)?;

        let mut fromid: u8 = 0;
        sleep(Duration::from_millis(420));

        if readReply {
            match readMessage(&mut port, &mut retmsg, &mut fromid) {
                Ok(true) => {
                    println!("reply from: {}", fromid);
                    // for i in 0..retmsg.buf.len() {
                    //     let c = retmsg.buf[i];
                    //     println!("{} - {}", c, c as char);
                    // }
                    am::print_payload(&retmsg.payload);
                }
                Ok(false) => {
                    println!("here");
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            }
        } else {
            let mut monobuf = [0; 1];
            let mut count = 0;
            while port.read_exact(&mut monobuf).is_ok() {
                // just print the chars we read.  good for debug from Serial.print() on the automato.
                // print!("{}", monobuf[0] as char);

                // println!("{} '{}'", monobuf[0] as u8, monobuf[0] as char);

                // print the index, number, and char
                println!("{} - {} - {}", count, monobuf[0] as u8, monobuf[0] as char);
                count = count + 1;
            }
            // let mut buf = String::new();
            // let mut monobuf = [0; 1];
            // port.read_exact(&mut monobuf)?;
            // buf.push(monobuf[0] as char);
            // if monobuf[0] as char == '\n' {
            //     println!("msg: {}", buf);
            // }
        }
    }
    Ok(())
}

unsafe fn writeMessage(
    port: &mut serial::SystemPort,
    msg: &Msgbuf,
    toid: u8,
) -> Result<(), serial::Error> {
    let sz = am::payloadSize(&msg.payload);

    port.write(&['m' as u8])?;
    port.write(&[toid as u8])?;
    port.write(&[sz as u8])?;
    port.write(&msg.buf[0..sz + 1])?;

    Ok(())
}

unsafe fn readMessage(
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

    if (sz > 0) {
        port.read_exact(&mut msg.buf[0..sz])?;
    }

    Ok(true)
}

unsafe fn readMsgFile(name: &str) -> Result<(), Box<dyn Error>> {
    let mut mfile = File::open(name)?;

    println!("");
    println!("reading: {}", name);

    // message with dummy payload.
    let mut mutmsg = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck,
            data: PayloadData { pin: 0 },
        },
    };

    mfile.read(&mut mutmsg.buf);

    // for i in 0..mutmsg.buf.len() {
    //     let c = mutmsg.buf[i];
    //     println!("{} - {}", c, c as char);
    // }
    am::print_payload(&mutmsg.payload);

    Ok(())
}
