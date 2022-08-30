use crate::config::Config;
// use crate::data::{};
use crate::messages::{PublicMessage, ServerResponse};
// use crate::sqldata;
use crate::data::{AutomatoMsg, ListAutomato, ServerData};
use automato::automatomsg as am;
use log::info;
use serial::SerialPort;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

// public json msgs don't require login.
pub fn public_interface(
    data: &ServerData,
    msg: PublicMessage,
) -> Result<ServerResponse, Box<dyn Error + '_>> {
    info!("process_public_json, what={}", msg.what.as_str());
    match msg.what.as_str() {
        "GetAutomatoList" => Ok(ServerResponse {
            what: "automatos".to_string(),
            content: serde_json::to_value(data.config.automato_ids.clone())?,
        }),
        "AutomatoMsg" => {
            let msgdata = Option::ok_or(msg.data.as_ref(), "malformed json data")?;
            let am: AutomatoMsg = serde_json::from_value(msgdata.clone())?;
            let mut mb = am::Msgbuf {
                buf: [0; am::RH_RF95_MAX_MESSAGE_LEN],
            };

            println!("automatomsg: {:?}", am);

            let mut retmsg = mb.clone();

            unsafe {
                mb.payload = am::Payload::from(am.message);
                let mut port = data.port.lock()?;
                am::write_message(&mut port, &mb, am.id)?;

                let mut fromid: u8 = 0;
                port.set_timeout(Duration::from_millis(420))?;

                match am::read_message(&mut port, &mut retmsg, &mut fromid) {
                    Ok(true) => {
                        println!("reply from: {}", fromid);
                        // for i in 0..retmsg.buf.len() {
                        //     let c = retmsg.buf[i];
                        //     println!("{} - {}", c, c as char);
                        // }
                        am::print_payload(&retmsg.payload);

                        let rm = AutomatoMsg {
                            id: fromid,
                            message: am::PayloadEnum::from(retmsg.payload),
                        };
                        Ok(ServerResponse {
                            what: "automatomsg".to_string(),
                            content: serde_json::to_value(rm)?,
                        })
                    }
                    Ok(false) => {
                        println!("here");
                        Ok(ServerResponse {
                            what: "here".to_string(),
                            content: serde_json::Value::Null,
                        })
                    }
                    Err(e) => {
                        println!("error: {:?}", e);
                        Ok(ServerResponse {
                            what: "err".to_string(),
                            content: serde_json::Value::Null,
                        })
                    }
                }
            }
        }
        wat => Err(Box::new(simple_error::SimpleError::new(format!(
            "invalid 'what' code:'{}'",
            wat
        )))),
    }
}
