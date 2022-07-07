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
                unsafe { write_message_files(&dir, write_json_message)? };
            }
            (Some(dir), Some(("read", _sub_matches))) => unsafe {
                println!("reading json files");
                read_message_files(&dir, read_msg_file_js)?;
            },
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
                unsafe { write_message_files(&dir, write_bin_message)? };
            }
            (Some(dir), Some(("read", _sub_matches))) => unsafe {
                println!("reading files");
                read_message_files(&dir, read_msg_file_bin)?;
            },
            meh => {
                bail!("unhandled command! {:?}", meh)
            }
        }
    }

    Ok(())
}

fn write_json_message(
    dir: &str,
    filename: &str,
    payload: Payload,
) -> Result<(), Box<dyn std::error::Error>> {
    let fname = format!("{}/{}.js", dir, filename);
    let v = serde_json::to_value(payload)?;
    let mut onfile = File::create(fname.as_str())?;
    onfile.write(v.to_string().as_bytes())?;
    Ok(())
}

unsafe fn write_bin_message(
    dir: &str,
    filename: &str,
    payload: Payload,
) -> Result<(), Box<dyn std::error::Error>> {
    let msgbuf = Msgbuf { payload: payload };
    let fname = format!("{}/{}.bin", dir, filename);
    let mut onfile = File::create(fname.as_str())?;
    onfile.write(&msgbuf.buf[0..am::payload_size(&msgbuf.payload)])?;
    Ok(())
}

unsafe fn write_message_files(
    dir: &str,
    write_message: unsafe fn(&str, &str, Payload) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut payload = Payload {
        payload_type: PayloadType::PtAck,
        data: PayloadData { unit: () },
    };

    am::setup_ack(&mut payload);
    write_message(dir, "ack.js", payload)?;

    am::setup_fail(&mut payload, ResultCode::RcInvalidRhRouterError);
    write_message(dir, "fail.js", payload)?;

    am::setup_pinmode(&mut payload, 26, 2);
    write_message(dir, "pinmode.js", payload)?;

    am::setup_readpin(&mut payload, 22);
    write_message(dir, "readpin.js", payload)?;

    am::setup_readpinreply(&mut payload, 26, 1);
    write_message(dir, "readpinreply.js", payload)?;

    am::setup_writepin(&mut payload, 15, 1);
    write_message(dir, "writepin.js", payload)?;

    am::setup_readanalog(&mut payload, 27);
    write_message(dir, "readanalog.js", payload)?;

    am::setup_readanalogreply(&mut payload, 6, 500);
    write_message(dir, "readanalogreply.js", payload)?;

    am::setup_readmem(&mut payload, 1500, 75);
    write_message(dir, "readmem.js", payload)?;

    let test = [1, 2, 3, 4, 5];
    am::setup_readmemreply(&mut payload, &test);
    write_message(dir, "readmemreply.js", payload)?;

    let test = [5, 4, 3, 2, 1];
    am::setup_writemem(&mut payload, 5678, &test);
    write_message(dir, "writemem.js", payload)?;

    am::setup_readinfo(&mut payload);
    write_message(dir, "readinfo.js", payload)?;

    am::setup_readinforeply(&mut payload, 1.1, 5678, 5000, 5);
    write_message(dir, "readinforeply.js", payload)?;

    am::setup_readhumidity(&mut payload);
    write_message(dir, "readhumidity.js", payload)?;

    am::setup_readhumidityreply(&mut payload, 45.7);
    write_message(dir, "readhumidityreply.js", payload)?;

    am::setup_readtemperature(&mut payload);
    write_message(dir, "readtemperature.js", payload)?;

    am::setup_readtemperaturereply(&mut payload, 98.6);
    write_message(dir, "readtemperaturereply.js", payload)?;

    am::setup_readfield(&mut payload, 1);
    write_message(dir, "readfield.js", payload)?;

    am::setup_readfieldreply(&mut payload, 7, 77, 20, 4, "wat".as_bytes());
    write_message(dir, "readfieldreply.js", payload)?;

    Ok(())
}

unsafe fn read_msg_file_bin(
    dir: &str,
    name: &str,
    msgbuf: &mut Msgbuf,
) -> Result<(), Box<dyn Error>> {
    let fname = format!("{}/{}.bin", dir, name);
    let mut mfile = File::open(fname.as_str())?;

    println!("");
    println!("reading: {}", fname);

    mfile.read(&mut msgbuf.buf)?;

    // for i in 0..msgbuf.buf.len() {
    //     let c = msgbuf.buf[i];
    //     println!("{} - {}", c, c as char);
    // }
    am::print_payload(&msgbuf.payload);

    Ok(())
}

unsafe fn read_msg_file_js(
    dir: &str,
    name: &str,
    msgbuf: &mut Msgbuf,
) -> Result<(), Box<dyn Error>> {
    let fname = format!("{}/{}.js", dir, name);
    println!("");
    println!("reading: {}", fname);

    let mut mfile = File::open(fname.as_str())?;

    let mut s = String::new();
    mfile.read_to_string(&mut s)?;
    let v: serde_json::Value = serde_json::from_str(s.as_str())?;
    let p: am::Payload = serde_json::from_value(v)?;

    msgbuf.payload = p;

    // for i in 0..msgbuf.buf.len() {
    //     let c = msgbuf.buf[i];
    //     println!("{} - {}", c, c as char);
    // }
    // am::print_payload(&msgbuf.payload);

    Ok(())
}

unsafe fn read_message_files(
    dir: &str,
    read_msg_file: unsafe fn(&str, &str, &mut Msgbuf) -> Result<(), Box<dyn Error>>,
) -> Result<bool, Box<dyn Error>> {
    // message with dummy payload.
    let mut mb = Msgbuf {
        payload: Payload {
            payload_type: am::PayloadType::PtAck,
            data: PayloadData { pin: 0 },
        },
    };

    read_msg_file(dir, "ack", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtAck {
        println!("ack msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "fail", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtFail
        || mb.payload.data.failcode != ResultCode::RcInvalidRhRouterError as u8
    {
        println!("fail msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "pinmode", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtPinmode
        || mb.payload.data.pinmode.pin != 26
        || mb.payload.data.pinmode.mode != 2
    {
        println!("pinmode msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readpin", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadpin || mb.payload.data.pin != 22 {
        println!("readpin msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readpinreply", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadpinreply
        || mb.payload.data.pinval.pin != 26
        || mb.payload.data.pinval.state != 1
    {
        println!("readpin msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "writepin", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtWritepin
        || mb.payload.data.pinval.pin != 15
        || mb.payload.data.pinval.state != 1
    {
        println!("writepin msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readanalog", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadanalog || mb.payload.data.pin != 27 {
        println!("readanalog msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readanalogreply", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadanalogreply
        || mb.payload.data.analogpinval.pin != 6
        || mb.payload.data.analogpinval.state != 500
    {
        println!("readanalogreply msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readmem", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadmem
        || mb.payload.data.readmem.address != 1500
        || mb.payload.data.readmem.length != 75
    {
        println!("readmem msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readmemreply", &mut mb)?;
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

    read_msg_file(dir, "writemem", &mut mb)?;
    let testwm = [5, 4, 3, 2, 1];
    if mb.payload.data.writemem.address != 5678
        || mb.payload.data.writemem.length != 5
        || mb.payload.data.writemem.data[0..5] != testwm
    {
        println!("bad writemem data");
        return Ok(false);
    }

    read_msg_file(dir, "readinfo", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadinfo {
        println!("readinfo msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readinforeply", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadinforeply
        || (mb.payload.data.remoteinfo.protoversion - 1.1) > 0.00000001
        || mb.payload.data.remoteinfo.mac_address != 5678
        || mb.payload.data.remoteinfo.datalen != 5000
        || mb.payload.data.remoteinfo.fieldcount != 5
    {
        println!("readinforeply msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readhumidity", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadhumidity {
        println!("readhumidity msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readhumidityreply", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadhumidityreply
        || (mb.payload.data.f - 45.7) > 0.000001
    {
        println!("readhumidityreply msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readtemperature", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadtemperature {
        println!("readtemperature msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readtemperaturereply", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadtemperaturereply
        || (mb.payload.data.f - 98.6) > 0.000001
    {
        println!("readtemperaturereply msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readfield", &mut mb)?;
    if mb.payload.payload_type != PayloadType::PtReadfield || mb.payload.data.readfield.index != 1 {
        println!("readfield msg failed");
        return Ok(false);
    }

    read_msg_file(dir, "readfieldreply", &mut mb)?;
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
