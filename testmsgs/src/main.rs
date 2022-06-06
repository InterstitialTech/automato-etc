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
    match (matches.value_of("dir"), matches.subcommand()) {
        (Some(dir), Some(("write", sub_matches))) => {
            println!("writing files");
            writeMessageFiles(&dir)?;
        }
        (Some(dir), Some(("read", sub_matches))) => unsafe {
            println!("reading files");
            readMessageFiles(&dir)?;
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

fn writeMessageFiles(dir: &str) -> Result<(), serial::Error> {
    let mut mutmsg = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck,
            data: PayloadData { pin: 0 },
        },
    };

    unsafe {
        am::setup_ack(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/ack.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_ack: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_fail(&mut mutmsg.payload, ResultCode::RcInvalidRhRouterError);
        let mut onfile = File::create(format!("{}/fail.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_fail: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_pinmode(&mut mutmsg.payload, 26, 2);
        let mut onfile = File::create(format!("{}/pinmode.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_pinmode: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readpin(&mut mutmsg.payload, 22);
        let mut onfile = File::create(format!("{}/readpin.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readpin: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readpinreply(&mut mutmsg.payload, 26, 1);
        let mut onfile = File::create(format!("{}/readpinreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readpinreply: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_writepin(&mut mutmsg.payload, 15, 1);
        let mut onfile = File::create(format!("{}/writepin.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_writepin: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readanalog(&mut mutmsg.payload, 27);
        let mut onfile = File::create(format!("{}/readanalog.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readanalog: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readanalogreply(&mut mutmsg.payload, 6, 500);
        let mut onfile = File::create(format!("{}/readanalogreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readanalogreply: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readmem(&mut mutmsg.payload, 1500, 75);
        let mut onfile = File::create(format!("{}/readmem.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readmem: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        let test = [1, 2, 3, 4, 5];
        am::setup_readmemreply(&mut mutmsg.payload, &test);
        let mut onfile = File::create(format!("{}/readmemreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readmemreply: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        let test = [5, 4, 3, 2, 1];
        am::setup_writemem(&mut mutmsg.payload, 5678, &test);
        let mut onfile = File::create(format!("{}/writemem.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_writemem: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readinfo(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/readinfo.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readinfo: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readinforeply(&mut mutmsg.payload, 1.1, 5678, 5000, 5);
        let mut onfile = File::create(format!("{}/readinforeply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readinforeply: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readhumidity(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/readhumidity.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readhumidity: {}", am::payloadSize(&mutmsg.payload));
    }

    unsafe {
        am::setup_readhumidityreply(&mut mutmsg.payload, 45.7);
        let mut onfile = File::create(format!("{}/readhumidityreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readhumidityreply: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readtemperature(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/readtemperature.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperature: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readtemperaturereply(&mut mutmsg.payload, 98.6);
        let mut onfile = File::create(format!("{}/readtemperaturereply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!(
            "setup_readtemperaturereply: {}",
            am::payloadSize(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readfield(&mut mutmsg.payload, 1);
        let mut onfile = File::create(format!("{}/readfield.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readfield: {}", am::payloadSize(&mutmsg.payload));
    };

    unsafe {
        am::setup_readfieldreply(&mut mutmsg.payload, 7, 77, 20, 4, "wat".as_bytes());
        let mut onfile = File::create(format!("{}/readfieldreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payloadSize(&mutmsg.payload)])?;

        println!("setup_readfieldreply: {}", am::payloadSize(&mutmsg.payload));
    };

    Ok(())
}

unsafe fn readMsgFile(name: &str, msgbuf: &mut Msgbuf) -> Result<(), Box<dyn Error>> {
    let mut mfile = File::open(name)?;

    println!("");
    println!("reading: {}", name);

    mfile.read(&mut msgbuf.buf);

    // for i in 0..msgbuf.buf.len() {
    //     let c = msgbuf.buf[i];
    //     println!("{} - {}", c, c as char);
    // }
    am::print_payload(&msgbuf.payload);

    Ok(())
}

unsafe fn readMessageFiles(dir: &str) -> Result<bool, Box<dyn Error>> {
    // message with dummy payload.
    let mut mb = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck,
            data: PayloadData { pin: 0 },
        },
    };

    readMsgFile(format!("{}/ack.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtAck) {
        println!("ack msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/fail.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtFail
        || mb.payload.data.failcode != ResultCode::RcInvalidRhRouterError as u8)
    {
        println!("fail msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/pinmode.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtPinmode
        || mb.payload.data.pinmode.pin != 26
        || mb.payload.data.pinmode.mode != 2)
    {
        println!("pinmode msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readpin.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadpin || mb.payload.data.pin != 22) {
        println!("readpin msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readpinreply.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadpinreply
        || mb.payload.data.pinval.pin != 26
        || mb.payload.data.pinval.state != 1)
    {
        println!("readpin msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/writepin.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtWritepin
        || mb.payload.data.pinval.pin != 15
        || mb.payload.data.pinval.state != 1)
    {
        println!("writepin msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readanalog.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadanalog || mb.payload.data.pin != 27) {
        println!("readanalog msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readanalogreply.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadanalogreply
        || mb.payload.data.analogpinval.pin != 6
        || mb.payload.data.analogpinval.state != 500)
    {
        println!("readanalogreply msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readmem.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadmem
        || mb.payload.data.readmem.address != 1500
        || mb.payload.data.readmem.length != 75)
    {
        println!("readmem msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readmemreply.bin", dir).as_str(), &mut mb)?;

    let testrm = [1, 2, 3, 4, 5];

    readMsgFile(format!("{}/writemem.bin", dir).as_str(), &mut mb)?;

    let testwm = [5, 4, 3, 2, 1];

    readMsgFile(format!("{}/readinfo.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadinfo) {
        println!("readinfo msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readinforeply.bin", dir).as_str(), &mut mb)?;

    println!(
        "wat: {}",
        mb.payload.payload_type != PayloadType::PtReadinforeply
    );
    println!(
        "wat: {}",
        (mb.payload.data.remoteinfo.protoversion - 1.1) > 0.00000001
    );
    println!("wat: {}", mb.payload.data.remoteinfo.macAddress != 5678);
    println!("wat: {}", mb.payload.data.remoteinfo.datalen != 5000);
    println!("wat: {}", mb.payload.data.remoteinfo.fieldcount != 5);
    if (mb.payload.payload_type != PayloadType::PtReadinforeply
        || (mb.payload.data.remoteinfo.protoversion - 1.1) > 0.00000001
        || mb.payload.data.remoteinfo.macAddress != 5678
        || mb.payload.data.remoteinfo.datalen != 5000
        || mb.payload.data.remoteinfo.fieldcount != 5)
    {
        println!("readinforeply msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readhumidity.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadhumidity) {
        println!("readhumidity msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readhumidityreply.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadhumidityreply
        || (mb.payload.data.f - 45.7) > 0.000001)
    {
        println!("readhumidityreply msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readtemperature.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadtemperature) {
        println!("readtemperature msg failed");
        return Ok(false);
    }

    readMsgFile(
        format!("{}/readtemperaturereply.bin", dir).as_str(),
        &mut mb,
    )?;

    if (mb.payload.payload_type != PayloadType::PtReadtemperaturereply
        || (mb.payload.data.f - 98.6) > 0.000001)
    {
        println!("readtemperaturereply msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readfield.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadfield || mb.payload.data.readfield.index != 1)
    {
        println!("readfield msg failed");
        return Ok(false);
    }

    readMsgFile(format!("{}/readfieldreply.bin", dir).as_str(), &mut mb)?;

    if (mb.payload.payload_type != PayloadType::PtReadfieldreply
        || mb.payload.data.readfieldreply.index != 7
        || mb.payload.data.readfieldreply.offset != 77
        || mb.payload.data.readfieldreply.length != 20
    // TODO: define format codes
        || mb.payload.data.readfieldreply.format != 4)
    {
        println!("readfieldreply msg failed");
        return Ok(false);
    }

    println!("-----------------------------------");
    println!("all tests passed!");

    Ok(true)
}
