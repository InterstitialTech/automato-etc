use automato::automatomsg as am;
use automato::automatomsg::{Msgbuf, Payload, PayloadData, PayloadType, ResultCode};
use clap::{Arg, Command};
use serde_json;
use simple_error::bail;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    match err_main() {
        Ok(()) => (),
        Err(e) => {
            println!("error: {:?}", e)
        }
    }
}

fn err_main() -> Result<(), Box<dyn Error>> {
    let matches = clap::Command::new("testmsgs")
        .version("1.0")
        .author("Automato Enterprises")
        .about("cli for testing automato messsages to/from files.")
        .arg(
            Arg::new("dir")
                .short('d')
                .long("directory")
                .value_name("<director>")
                .help("directory to write files to, or read files from.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("read/write messages in json form")
                .takes_value(false),
        )
        .subcommand_required(true)
        .subcommand(Command::new("write").about("write files"))
        .subcommand(Command::new("read").about("read files"))
        .get_matches();

    let json = matches.is_present("json");

    if json {
        // set up the outgoing message.
        match (matches.value_of("dir"), matches.subcommand()) {
            (Some(dir), Some(("write", _sub_matches))) => {
                println!("writing json files");
                write_json_message_files(&dir)?;
            }
            (Some(dir), Some(("read", _sub_matches))) => {
                println!("reading json files");
                read_json_message_files(&dir)?;
            }
            meh => {
                bail!("unhandled command! {:?}", meh)
            }
        }
    } else {
        println!("json: {}", json);

        // set up the outgoing message.
        match (matches.value_of("dir"), matches.subcommand()) {
            (Some(dir), Some(("write", _sub_matches))) => {
                println!("writing files");
                write_message_files(&dir)?;
            }
            (Some(dir), Some(("read", _sub_matches))) => unsafe {
                println!("reading files");
                read_message_files(&dir)?;
            },
            meh => {
                bail!("unhandled command! {:?}", meh)
            }
        }
    }

    Ok(())
}

fn write_message_files(dir: &str) -> Result<(), serial::Error> {
    let mut mutmsg = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck,
            data: PayloadData { pin: 0 },
        },
    };

    unsafe {
        am::setup_ack(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/ack.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_ack: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_fail(&mut mutmsg.payload, ResultCode::RcInvalidRhRouterError);
        let mut onfile = File::create(format!("{}/fail.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_fail: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_pinmode(&mut mutmsg.payload, 26, 2);
        let mut onfile = File::create(format!("{}/pinmode.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_pinmode: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readpin(&mut mutmsg.payload, 22);
        let mut onfile = File::create(format!("{}/readpin.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readpin: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readpinreply(&mut mutmsg.payload, 26, 1);
        let mut onfile = File::create(format!("{}/readpinreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readpinreply: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_writepin(&mut mutmsg.payload, 15, 1);
        let mut onfile = File::create(format!("{}/writepin.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_writepin: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readanalog(&mut mutmsg.payload, 27);
        let mut onfile = File::create(format!("{}/readanalog.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readanalog: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readanalogreply(&mut mutmsg.payload, 6, 500);
        let mut onfile = File::create(format!("{}/readanalogreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!(
            "setup_readanalogreply: {}",
            am::payload_size(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readmem(&mut mutmsg.payload, 1500, 75);
        let mut onfile = File::create(format!("{}/readmem.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readmem: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        let test = [1, 2, 3, 4, 5];
        am::setup_readmemreply(&mut mutmsg.payload, &test);
        let mut onfile = File::create(format!("{}/readmemreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readmemreply: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        let test = [5, 4, 3, 2, 1];
        am::setup_writemem(&mut mutmsg.payload, 5678, &test);
        let mut onfile = File::create(format!("{}/writemem.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_writemem: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readinfo(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/readinfo.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readinfo: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readinforeply(&mut mutmsg.payload, 1.1, 5678, 5000, 5);
        let mut onfile = File::create(format!("{}/readinforeply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readinforeply: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readhumidity(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/readhumidity.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readhumidity: {}", am::payload_size(&mutmsg.payload));
    }

    unsafe {
        am::setup_readhumidityreply(&mut mutmsg.payload, 45.7);
        let mut onfile = File::create(format!("{}/readhumidityreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!(
            "setup_readhumidityreply: {}",
            am::payload_size(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readtemperature(&mut mutmsg.payload);
        let mut onfile = File::create(format!("{}/readtemperature.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!(
            "setup_readtemperature: {}",
            am::payload_size(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readtemperaturereply(&mut mutmsg.payload, 98.6);
        let mut onfile = File::create(format!("{}/readtemperaturereply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!(
            "setup_readtemperaturereply: {}",
            am::payload_size(&mutmsg.payload)
        );
    }

    unsafe {
        am::setup_readfield(&mut mutmsg.payload, 1);
        let mut onfile = File::create(format!("{}/readfield.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!("setup_readfield: {}", am::payload_size(&mutmsg.payload));
    };

    unsafe {
        am::setup_readfieldreply(&mut mutmsg.payload, 7, 77, 20, 4, "wat".as_bytes());
        let mut onfile = File::create(format!("{}/readfieldreply.bin", dir).as_str())?;
        onfile.write(&mutmsg.buf[0..am::payload_size(&mutmsg.payload)])?;

        println!(
            "setup_readfieldreply: {}",
            am::payload_size(&mutmsg.payload)
        );
    };

    Ok(())
}

fn write_json_message_files(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let mut mutmsg = Msgbuf {
    //     payload: Payload {
    //         payload_type: am::PayloadType::PtAck,
    //         data: PayloadData { pin: 0 },
    //     },
    // };

    let mut payload = Payload {
        payload_type: PayloadType::PtAck,
        data: PayloadData { unit: () },
    };

    {
        let test = [5, 4, 3, 2, 1];
        am::setup_writemem(&mut payload, 5678, &test);
        let v = serde_json::to_value(payload)?;
        let mut onfile = File::create(format!("{}/writemem.js", dir).as_str())?;
        onfile.write(v.to_string().as_bytes())?;

        println!("setup_writemem: {}", am::payload_size(&payload));
    }

    {
        let test = [1, 2, 3, 4, 5];
        am::setup_readmemreply(&mut payload, &test);
        let mut onfile = File::create(format!("{}/readmemreply.js", dir).as_str())?;

        let v = serde_json::to_value(payload)?;
        onfile.write(v.to_string().as_bytes())?;
        println!("setup_readmemreply: {}", am::payload_size(&payload));
    }

    // unsafe {
    //     let test = [5, 4, 3, 2, 1];
    //     am::setup_writemem(&mut mutmsg.payload, 5678, &test);
    //     let mut onfile = File::create(format!("{}/writemem.js", dir).as_str())?;
    //     let v = serde_json::to_value(mutmsg.payload.data.writemem)?;
    //     onfile.write(v.to_string().as_bytes())?;

    //     println!("setup_writemem: {}", am::payload_size(&mutmsg.payload));
    // }

    Ok(())
}

fn read_json_message_files(dir: &str) -> Result<bool, Box<dyn std::error::Error>> {
    {
        let mut mfile = File::open(format!("{}/writemem.js", dir).as_str())?;
        let mut s = String::new();
        mfile.read_to_string(&mut s)?;
        println!("json input: {}", s);
        let v: serde_json::Value = serde_json::from_str(s.as_str())?;
        let p: am::Payload = serde_json::from_value(v)?;
        println!("p: {:?} ", serde_json::to_value(p));
        println!("");
    }
    {
        let mut mfile = File::open(format!("{}/readmemreply.js", dir).as_str())?;
        let mut s = String::new();
        mfile.read_to_string(&mut s)?;
        println!("json input: {}", s);
        let v: serde_json::Value = serde_json::from_str(s.as_str())?;
        let p: am::Payload = serde_json::from_value(v)?;
        println!("p: {:?} ", serde_json::to_value(p));
        println!("");
    }
    Ok(true)
}

unsafe fn read_msg_file(name: &str, msgbuf: &mut Msgbuf) -> Result<(), Box<dyn Error>> {
    let mut mfile = File::open(name)?;

    println!("");
    println!("reading: {}", name);

    mfile.read(&mut msgbuf.buf)?;

    // for i in 0..msgbuf.buf.len() {
    //     let c = msgbuf.buf[i];
    //     println!("{} - {}", c, c as char);
    // }
    am::print_payload(&msgbuf.payload);

    Ok(())
}

unsafe fn read_message_files(dir: &str) -> Result<bool, Box<dyn Error>> {
    // message with dummy payload.
    let mut mb = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck,
            data: PayloadData { pin: 0 },
        },
    };

    read_msg_file(format!("{}/ack.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtAck {
        println!("ack msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/fail.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtFail
        || mb.payload.data.failcode != ResultCode::RcInvalidRhRouterError as u8
    {
        println!("fail msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/pinmode.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtPinmode
        || mb.payload.data.pinmode.pin != 26
        || mb.payload.data.pinmode.mode != 2
    {
        println!("pinmode msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readpin.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadpin || mb.payload.data.pin != 22 {
        println!("readpin msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readpinreply.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadpinreply
        || mb.payload.data.pinval.pin != 26
        || mb.payload.data.pinval.state != 1
    {
        println!("readpin msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/writepin.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtWritepin
        || mb.payload.data.pinval.pin != 15
        || mb.payload.data.pinval.state != 1
    {
        println!("writepin msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readanalog.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadanalog || mb.payload.data.pin != 27 {
        println!("readanalog msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readanalogreply.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadanalogreply
        || mb.payload.data.analogpinval.pin != 6
        || mb.payload.data.analogpinval.state != 500
    {
        println!("readanalogreply msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readmem.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadmem
        || mb.payload.data.readmem.address != 1500
        || mb.payload.data.readmem.length != 75
    {
        println!("readmem msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readmemreply.bin", dir).as_str(), &mut mb)?;
    let testrm = [1, 2, 3, 4, 5];

    if mb.payload.data.readmemreply.length != 5 {
        println!(
            "invalid readmemreply length: {}",
            mb.payload.data.readmemreply.length
        );
        return Ok(false);
    }

    if mb.payload.data.readmemreply.data[0..5] != testrm {
        println!("data payload doesn't match!");
        return Ok(false);
    }

    read_msg_file(format!("{}/writemem.bin", dir).as_str(), &mut mb)?;

    let testwm = [5, 4, 3, 2, 1];
    if mb.payload.data.writemem.address != 5678
        || mb.payload.data.writemem.length != 5
        || mb.payload.data.writemem.data[0..5] != testwm
    {
        println!("bad writemem data");
        return Ok(false);
    }

    read_msg_file(format!("{}/readinfo.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadinfo {
        println!("readinfo msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readinforeply.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadinforeply
        || (mb.payload.data.remoteinfo.protoversion - 1.1) > 0.00000001
        || mb.payload.data.remoteinfo.mac_address != 5678
        || mb.payload.data.remoteinfo.datalen != 5000
        || mb.payload.data.remoteinfo.fieldcount != 5
    {
        println!("readinforeply msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readhumidity.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadhumidity {
        println!("readhumidity msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readhumidityreply.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadhumidityreply
        || (mb.payload.data.f - 45.7) > 0.000001
    {
        println!("readhumidityreply msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readtemperature.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadtemperature {
        println!("readtemperature msg failed");
        return Ok(false);
    }

    read_msg_file(
        format!("{}/readtemperaturereply.bin", dir).as_str(),
        &mut mb,
    )?;

    if mb.payload.payload_type != PayloadType::PtReadtemperaturereply
        || (mb.payload.data.f - 98.6) > 0.000001
    {
        println!("readtemperaturereply msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readfield.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadfield || mb.payload.data.readfield.index != 1 {
        println!("readfield msg failed");
        return Ok(false);
    }

    read_msg_file(format!("{}/readfieldreply.bin", dir).as_str(), &mut mb)?;

    if mb.payload.payload_type != PayloadType::PtReadfieldreply
        || mb.payload.data.readfieldreply.index != 7
        || mb.payload.data.readfieldreply.offset != 77
        || mb.payload.data.readfieldreply.length != 20
    // TODO: define format codes
        || mb.payload.data.readfieldreply.format != 4
    {
        println!("readfieldreply msg failed");
        return Ok(false);
    }

    println!("-----------------------------------");
    println!("all tests passed!");

    Ok(true)
}
