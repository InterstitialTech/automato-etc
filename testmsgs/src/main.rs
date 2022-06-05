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
    let matches = clap::App::new("testmsgs")
        .version("1.0")
        .author("Automato Enterprises")
        .about("cli for testing automato messsages to/from files.")
        .arg(
            Arg::with_name("dir")
                .short('d')
                .long("directory")
                .value_name("<director>")
                .help("directory to write files to, or read files from.")
                .required(true)
                .takes_value(true),
        )
        .subcommand_required(true)
        .subcommand(Command::new("write").about("write files"))
        .subcommand(Command::new("read").about("read files"))
        .get_matches();

    // set up the outgoing message.
    match matches.subcommand() {
        Some(("write", sub_matches)) => {
            writeMessageFiles();
        }
        Some(("read", sub_matches)) => unsafe {
            match readMessageFiles() {
                Ok(()) => (),
                Err(e) => {
                    println!("error: {:?}", e)
                }
            }
        },
        meh => {
            bail!("unhandled command! {:?}", meh)
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

fn writeMessageFiles() -> Result<(), serial::Error> {
    let mut mutmsg = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck as u8,
            data: PayloadData { pin: 0 },
        },
    };

    unsafe {
        am::setup_ack(&mut mutmsg.payload);
        let mut onfile = File::create("rustmsgs-out/ack.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_ack: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_fail(&mut mutmsg.payload, ResultCode::RcInvalidRhRouterError);
        let mut onfile = File::create("rustmsgs-out/fail.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_fail: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_pinmode(&mut mutmsg.payload, 26, 2);
        let mut onfile = File::create("rustmsgs-out/pinmode.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_pinmode: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readpin(&mut mutmsg.payload, 22);
        let mut onfile = File::create("rustmsgs-out/readpin.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readpin: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readpinreply(&mut mutmsg.payload, 26, 1);
        let mut onfile = File::create("rustmsgs-out/readpinreply.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readpinreply: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_writepin(&mut mutmsg.payload, 15, 1);
        let mut onfile = File::create("rustmsgs-out/writepin.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_writepin: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readanalog(&mut mutmsg.payload, 27);
        let mut onfile = File::create("rustmsgs-out/readanalog.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readanalog: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readanalogreply(&mut mutmsg.payload, 6, 500);
        let mut onfile = File::create("rustmsgs-out/readanalogreply.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readanalogreply: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readmem(&mut mutmsg.payload, 1500, 75);
        let mut onfile = File::create("rustmsgs-out/readmem.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readmem: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        let test = [1, 2, 3, 4, 5];
        am::setup_readmemreply(&mut mutmsg.payload, &test);
        let mut onfile = File::create("rustmsgs-out/readmemreply.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readmemreply: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        let test = [5, 4, 3, 2, 1];
        am::setup_writemem(&mut mutmsg.payload, 5678, &test);
        let mut onfile = File::create("rustmsgs-out/writemem.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_writemem: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readinfo(&mut mutmsg.payload);
        let mut onfile = File::create("rustmsgs-out/readinfo.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readinfo: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readinforeply(&mut mutmsg.payload, 1.1, 5678, 5000, 5);
        let mut onfile = File::create("rustmsgs-out/readinforeply.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readinforeply: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readhumidity(&mut mutmsg.payload);
        let mut onfile = File::create("rustmsgs-out/readhumidity.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readhumidity: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readhumidityreply(&mut mutmsg.payload, 45.7);
        let mut onfile = File::create("rustmsgs-out/readhumidityreply.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readhumidityreply: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readtemperature(&mut mutmsg.payload);
        let mut onfile = File::create("rustmsgs-out/readtemperature.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperature: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readtemperaturereply(&mut mutmsg.payload, 98.6);
        let mut onfile = File::create("rustmsgs-out/readtemperaturereply.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperaturereply: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readfield(&mut mutmsg.payload, 1);
        let mut onfile = File::create("rustmsgs-out/readfield.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readfield: {}", am::payloadSize(&mutmsg.payload));
    };

    unsafe {
        am::setup_readfieldreply(&mut mutmsg.payload, 77, 20, 5, 2, "wat".as_bytes());
        let mut onfile = File::create("rustmsgs-out/readfieldreply.bin")?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readfieldreply: {}", am::payloadSize(&mutmsg.payload));
    };

    Ok(())
}

unsafe fn readMsgFile(name: &str) -> Result<(), Box<dyn Error>> {
    let mut mfile = File::open(name)?;

    println!("");
    println!("reading: {}", name);

    // message with dummy payload.
    let mut mutmsg = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck as u8,
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

unsafe fn readMessageFiles() -> Result<(), Box<dyn Error>> {
    readMsgFile("cmsgs-out/ack.bin")?;
    readMsgFile("cmsgs-out/fail.bin")?;
    readMsgFile("cmsgs-out/pinmode.bin")?;
    readMsgFile("cmsgs-out/readpin.bin")?;
    readMsgFile("cmsgs-out/readpinreply.bin")?;
    readMsgFile("cmsgs-out/writepin.bin")?;
    readMsgFile("cmsgs-out/readanalog.bin")?;
    readMsgFile("cmsgs-out/readanalogreply.bin")?;
    readMsgFile("cmsgs-out/readmem.bin")?;
    readMsgFile("cmsgs-out/readmemreply.bin")?;
    readMsgFile("cmsgs-out/writemem.bin")?;
    readMsgFile("cmsgs-out/readinfo.bin")?;
    readMsgFile("cmsgs-out/readinforeply.bin")?;
    readMsgFile("cmsgs-out/readhumidity.bin")?;
    readMsgFile("cmsgs-out/readhumidityreply.bin")?;
    readMsgFile("cmsgs-out/readtemperature.bin")?;
    readMsgFile("cmsgs-out/readtemperaturereply.bin")?;
    readMsgFile("cmsgs-out/readfield.bin")?;
    readMsgFile("cmsgs-out/readfieldreply.bin")?;

    Ok(())
}
