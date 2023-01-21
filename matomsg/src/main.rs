use automato::automatomsg as am;
use automato::automatomsg::Msgbuf;
use clap::{Arg, Command};
use simple_error::bail;
use std::error::Error;
use std::io::Read;
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
    let matches = clap::Command::new("matomsg")
        .version("1.0")
        .author("Automato Enterprises")
        .about("cli for testing automato messsages over serial.")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("FILE")
                .help("serial port")
                .default_value("/dev/ttyUSB0")
                .takes_value(true),
        )
        .arg(
            Arg::new("baud")
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
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("NUMBER")
                .help("timeout (ms)")
                .default_value("420")
                .takes_value(true),
        )
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("0-255")
                .help("lora network address of an automato")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("write message output in json format")
                .takes_value(false),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("writepin")
                .about("write 0 or 1 to pin")
                .arg(Arg::new("pin").value_name("PIN").takes_value(true))
                .arg(Arg::new("value").value_name("1 or 0").takes_value(true)),
        )
        .subcommand(
            Command::new("pinmode")
                .about("write 0 or 1 to pin")
                .arg(Arg::new("pin").value_name("PIN").takes_value(true))
                .arg(Arg::new("value").value_name("1 or 0").takes_value(true)),
        )
        .subcommand(
            Command::new("readpin")
                .about("query pin")
                .arg(Arg::new("pin").value_name("PIN").takes_value(true)),
        )
        .subcommand(
            Command::new("readanalog")
                .about("query pin")
                .arg(Arg::new("pin").value_name("PIN").takes_value(true)),
        )
        .subcommand(Command::new("readinfo").about("read automato general info"))
        .subcommand(Command::new("readhumidity").about("read automato humidity"))
        .subcommand(Command::new("readtemperature").about("read automato temperature"))
        .subcommand(
            Command::new("writemem")
                .about("write hex data to automato memory")
                .arg(Arg::new("address").value_name("NUMBER").takes_value(true))
                .arg(Arg::new("value").value_name("hex string").takes_value(true)),
        )
        .subcommand(
            Command::new("readmem")
                .about("read hex data from automato memory")
                .arg(Arg::new("address").value_name("NUMBER").takes_value(true))
                .arg(Arg::new("length").value_name("NUMBER").takes_value(true)),
        )
        .subcommand(
            Command::new("readfield")
                .about("read field info from automato memory map")
                .arg(Arg::new("index").value_name("NUMBER").takes_value(true)),
        )
        .get_matches();

    let (port, baud, automatoaddr, timeout) = match (
        matches.value_of("port"),
        matches.value_of("baud"),
        matches.value_of("address"),
        matches.value_of("timeout"),
    ) {
        (Some(port), Some(baudstr), Some(addrstr), Some(timeout)) => {
            let baud = BaudRate::from_speed(baudstr.parse::<usize>()?);
            let addr = addrstr.parse::<u8>()?;
            (port, baud, addr, timeout.parse::<u64>()?)
        }
        _ => bail!("arg failure"),
    };

    let json = matches.is_present("json");

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
        Some(("readinfo", _sub_matches)) => {
            unsafe { am::setup_readinfo(&mut mb.payload) };
        }
        Some(("readhumidity", _sub_matches)) => {
            unsafe { am::setup_readhumidity(&mut mb.payload) };
        }
        Some(("readtemperature", _sub_matches)) => {
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

    let debug_reply = false;
    unsafe {
        am::write_message(&mut port, &mb, automatoaddr)?;

        let mut fromid: u8 = 0;
        port.set_timeout(Duration::from_millis(timeout));

        if debug_reply {
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
        } else {
            match am::read_message(&mut port, &mut retmsg, &mut fromid) {
                Ok(true) => {
                    println!("reply from: {}", fromid);
                    // for i in 0..retmsg.buf.len() {
                    //     let c = retmsg.buf[i];
                    //     println!("{} - {}", c, c as char);
                    // }
                    if json {
                        println!("unimplemented");
                    } else {
                        am::print_payload(&retmsg.payload);
                    }
                }
                Ok(false) => {
                    println!("here");
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            }
        }
    }
    Ok(())
}
