use clap::{Arg, Command, SubCommand};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use simple_error::bail;
use std::io::{Read, Write};
mod automatomsg;
use automatomsg::{
    AnalogPinval, Message, Payload, PayloadData, PayloadType, Pinmode, Pinval, Readmem,
    ReadmemReply, RemoteInfo, ResultCode, Writemem,
};
use num_derive;
use std::error::Error;
use std::fs::File;
use std::mem::size_of;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use serial::{BaudRate, CharSize, FlowControl, Parity, PortSettings, SerialPort, StopBits};

fn main() {
    // unsafe {
    //     match readMessageFiles() {
    //         Ok(()) => (),
    //         Err(e) => {
    //             println!("error: {:?}", e)
    //         }
    //     }
    // }
    // writeMessageFiles();

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

    let mut mb = automatomsg::Msgbuf {
        buf: [0; automatomsg::RH_RF95_MAX_MESSAGE_LEN],
    };

    let mut retmsg = mb.clone();

    // set up the outgoing message.
    match matches.subcommand() {
        Some(("writepin", sub_matches)) => {
            let (pin, val) = match (sub_matches.value_of("pin"), sub_matches.value_of("value")) {
                (Some(pinstr), Some(valstr)) => (pinstr.parse::<u8>()?, valstr.parse::<u8>()?),
                _ => bail!("arg failure"),
            };

            unsafe { automatomsg::setup_writepin(&mut mb.payload, pin, val) };
        }
        Some(("pinmode", sub_matches)) => {
            let (pin, val) = match (sub_matches.value_of("pin"), sub_matches.value_of("value")) {
                (Some(pinstr), Some(valstr)) => (pinstr.parse::<u8>()?, valstr.parse::<u8>()?),
                _ => bail!("arg failure"),
            };

            unsafe { automatomsg::setup_pinmode(&mut mb.payload, pin, val) };
        }
        Some(("readpin", sub_matches)) => {
            let pin = match sub_matches.value_of("pin") {
                Some(pinstr) => pinstr.parse::<u8>()?,
                _ => bail!("arg failure"),
            };
            unsafe { automatomsg::setup_readpin(&mut mb.payload, pin) };
        }
        Some(("readanalog", sub_matches)) => {
            let pin = match sub_matches.value_of("pin") {
                Some(pinstr) => pinstr.parse::<u8>()?,
                _ => bail!("arg failure"),
            };
            unsafe { automatomsg::setup_readanalog(&mut mb.payload, pin) };
        }
        Some(("readinfo", sub_matches)) => {
            unsafe { automatomsg::setup_readinfo(&mut mb.payload) };
        }
        Some(("readhumidity", sub_matches)) => {
            unsafe { automatomsg::setup_readhumidity(&mut mb.payload) };
        }
        Some(("readtemperature", sub_matches)) => {
            unsafe { automatomsg::setup_readtemperature(&mut mb.payload) };
        }
        Some(("writemem", sub_matches)) => {
            let (addr, val) = match (
                sub_matches.value_of("address"),
                sub_matches.value_of("value"),
            ) {
                (Some(addrstr), Some(valstr)) => (addrstr.parse::<u16>()?, hex::decode(valstr)?),
                _ => bail!("arg failure"),
            };

            unsafe { automatomsg::setup_writemem(&mut mb.payload, addr, val.as_slice()) };
        }
        Some(("readmem", sub_matches)) => {
            let (addr, len) = match (
                sub_matches.value_of("address"),
                sub_matches.value_of("length"),
            ) {
                (Some(addrstr), Some(lenstr)) => (addrstr.parse::<u16>()?, lenstr.parse::<u8>()?),
                _ => bail!("arg failure"),
            };
            unsafe { automatomsg::setup_readmem(&mut mb.payload, addr, len) };
        }
        Some(("readfield", sub_matches)) => {
            let index = match sub_matches.value_of("index") {
                Some(istr) => istr.parse::<u16>()?,
                _ => bail!("arg failure"),
            };
            unsafe { automatomsg::setup_readfield(&mut mb.payload, index) };
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

    let readReply = false;
    unsafe {
        writeMessage(&mut port, &mb, automatoaddr)?;

        let mut fromid: u8 = 0;
        sleep(Duration::from_millis(120));

        if readReply {
            while match readMessage(&mut port, &mut retmsg, &mut fromid) {
                Ok(true) => {
                    println!("reply from: {}", fromid);
                    // for i in 0..retmsg.buf.len() {
                    //     let c = retmsg.buf[i];
                    //     println!("{} - {}", c, c as char);
                    // }
                    automatomsg::print_payload(&retmsg.payload);
                    true
                }
                Ok(false) => {
                    println!("here");
                    false
                }
                Err(e) => {
                    println!("error: {:?}", e);
                    false
                }
            } {}
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

fn old_main() -> Result<(), Box<dyn Error>> {
    let mut port = serial::open("/dev/ttyUSB0")?;

    let ps = PortSettings {
        baud_rate: BaudRate::Baud115200,
        char_size: CharSize::Bits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::Stop1,
        flow_control: FlowControl::FlowSoftware,
    };
    port.configure(&ps)?;

    let PIN_LED: u8 = 26;

    let onmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtWritepin as u8, // .to_u8(),
            data: PayloadData {
                pinval: Pinval {
                    pin: PIN_LED,
                    state: 1,
                },
            },
        },
    };
    let offmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtWritepin as u8,
            data: PayloadData {
                pinval: Pinval {
                    pin: PIN_LED,
                    state: 0,
                },
            },
        },
    };

    unsafe {
        println!("payload_type: {}", onmsg.payload.payload_type);
    }

    let mut onfile = File::create("onmsg.bin")?;
    unsafe {
        onfile.write(&onmsg.buf[0..6]);
    }

    let id: u8 = 1;

    let mut on = true;

    let mut retmsg = offmsg.clone();

    loop {
        unsafe {
            if on {
                writeMessage(&mut port, &onmsg, id)?;
            } else {
                writeMessage(&mut port, &offmsg, id)?;
            }

            let mut fromid: u8 = 0;

            readMessage(&mut port, &mut retmsg, &mut fromid);
        }

        on = !on;

        sleep(Duration::from_millis(20));
    }
}

unsafe fn writeMessage(
    port: &mut serial::SystemPort,
    msg: &automatomsg::Msgbuf,
    toid: u8,
) -> Result<(), serial::Error> {
    let sz = automatomsg::payloadSize(&msg.payload);

    port.write(&['m' as u8])?;
    port.write(&[toid as u8])?;
    port.write(&[sz as u8])?;
    port.write(&msg.buf[0..sz + 1])?;

    Ok(())
}

unsafe fn readMessage(
    port: &mut serial::SystemPort,
    msg: &mut automatomsg::Msgbuf,
    fromid: &mut u8,
) -> Result<bool, serial::Error> {
    let mut monobuf = [0; 1];
    port.read_exact(&mut monobuf)?;
    println!(
        "monobuf -'m': {} - {}",
        monobuf[0] as u8, monobuf[0] as char
    );
    if monobuf[0] as char != 'm' {
        return Ok(false);
    }
    port.read_exact(&mut monobuf)?;
    *fromid = monobuf[0];
    println!(
        "monobuf fromid: {} - {}",
        monobuf[0] as u8, monobuf[0] as char
    );

    port.read_exact(&mut monobuf)?;
    let sz = monobuf[0] as usize;
    println!(
        "monobuf size: {} - {}",
        monobuf[0] as u8, monobuf[0] as char
    );

    port.read_exact(&mut monobuf)?;
    let sz = monobuf[0] as usize;
    println!(
        "monobuf type: {} - {}",
        monobuf[0] as u8, monobuf[0] as char
    );
    msg.buf[0] = monobuf[0];

    if (sz > 0) {
        port.read_exact(&mut msg.buf[1..sz])?;
    }

    // port.read_exact(&mut msg.buf[0..sz])?;

    Ok(true)
}

fn writeMessageFiles() -> Result<(), serial::Error> {
    let mut mutmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtAck as u8,
            data: PayloadData { pin: 0 },
        },
    };

    unsafe {
        automatomsg::setup_ack(&mut mutmsg.payload);
        let mut onfile = File::create("ack.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!("setup_ack: {}", automatomsg::payloadSize(&mutmsg.payload));
    }

    unsafe {
        automatomsg::setup_fail(&mut mutmsg.payload, ResultCode::RcInvalidRhRouterError);
        let mut onfile = File::create("fail.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!("setup_fail: {}", automatomsg::payloadSize(&mutmsg.payload));
    }

    unsafe {
        automatomsg::setup_pinmode(&mut mutmsg.payload, 26, 2);
        let mut onfile = File::create("pinmode.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_pinmode: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readpin(&mut mutmsg.payload, 22);
        let mut onfile = File::create("readpin.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readpin: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readpinreply(&mut mutmsg.payload, 26, 1);
        let mut onfile = File::create("readpinreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readpinreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_writepin(&mut mutmsg.payload, 15, 1);
        let mut onfile = File::create("writepin.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_writepin: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readanalog(&mut mutmsg.payload, 27);
        let mut onfile = File::create("readanalog.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readanalog: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readanalogreply(&mut mutmsg.payload, 6, 500);
        let mut onfile = File::create("readanalogreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readanalogreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readmem(&mut mutmsg.payload, 1500, 75);
        let mut onfile = File::create("readmem.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readmem: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        let test = [1, 2, 3, 4, 5];
        automatomsg::setup_readmemreply(&mut mutmsg.payload, &test);
        let mut onfile = File::create("readmemreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readmemreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        let test = [5, 4, 3, 2, 1];
        automatomsg::setup_writemem(&mut mutmsg.payload, 5678, &test);
        let mut onfile = File::create("writemem.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_writemem: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readinfo(&mut mutmsg.payload);
        let mut onfile = File::create("readinfo.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readinfo: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readinforeply(&mut mutmsg.payload, 1.1, 5678, 5000, 7);
        let mut onfile = File::create("readinforeply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readinforeply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readhumidity(&mut mutmsg.payload);
        let mut onfile = File::create("readhumidity.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readhumidity: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readhumidityreply(&mut mutmsg.payload, 45.7);
        let mut onfile = File::create("readhumidityreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readhumidityreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readtemperature(&mut mutmsg.payload);
        let mut onfile = File::create("readtemperature.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperature: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readtemperaturereply(&mut mutmsg.payload, 98.6);
        let mut onfile = File::create("readtemperaturereply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperaturereply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        automatomsg::setup_readfield(&mut mutmsg.payload, 1);
        let mut onfile = File::create("readfield.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readfield: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    };

    unsafe {
        automatomsg::setup_readfieldreply(&mut mutmsg.payload, 1, 20, 5, "wat".as_bytes());
        let mut onfile = File::create("readfieldreply.bin")?;
        onfile.write(&mutmsg.buf[0..automatomsg::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readfieldreply: {}",
            automatomsg::payloadSize(&mutmsg.payload)
        );
    };

    Ok(())
}

unsafe fn readMsgFile(name: &str) -> Result<(), Box<dyn Error>> {
    let mut mfile = File::open(name)?;

    println!("");
    println!("reading: {}", name);

    // message with dummy payload.
    let mut mutmsg = automatomsg::Msgbuf {
        payload: Payload {
            payload_type: automatomsg::PayloadType::PtAck as u8,
            data: PayloadData { pin: 0 },
        },
    };

    mfile.read(&mut mutmsg.buf);

    // for i in 0..mutmsg.buf.len() {
    //     let c = mutmsg.buf[i];
    //     println!("{} - {}", c, c as char);
    // }
    automatomsg::print_payload(&mutmsg.payload);

    Ok(())
}

unsafe fn readMessageFiles() -> Result<(), Box<dyn Error>> {
    readMsgFile("ack.bin")?;
    readMsgFile("fail.bin")?;
    readMsgFile("pinmode.bin")?;
    readMsgFile("readpin.bin")?;
    readMsgFile("readpinreply.bin")?;
    readMsgFile("writepin.bin")?;
    readMsgFile("readanalog.bin")?;
    readMsgFile("readanalogreply.bin")?;
    readMsgFile("readmem.bin")?;
    readMsgFile("readmemreply.bin")?;
    readMsgFile("writemem.bin")?;
    readMsgFile("readinfo.bin")?;
    readMsgFile("readinforeply.bin")?;
    readMsgFile("readhumidity.bin")?;
    readMsgFile("readhumidityreply.bin")?;
    readMsgFile("readtemperature.bin")?;
    readMsgFile("readtemperaturereply.bin")?;
    readMsgFile("readfield.bin")?;
    readMsgFile("readfieldreply.bin")?;

    Ok(())
}
