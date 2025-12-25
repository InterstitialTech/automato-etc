use crate::data::ServerData;
use crate::messages::{get_port_info, AutomatoMsg};
use crate::messages::{PublicMessage, ServerResponse};
use crate::serial_error;
use automato::automatomsg as am;
use serialport::available_ports;
use std::error::Error;
use std::time::Duration;

// public json msgs don't require login.
pub fn public_interface(
    data: &ServerData,
    msg: PublicMessage,
) -> Result<ServerResponse, Box<dyn Error + '_>> {
    // info!("process_public_json {}", msg.as_str());
    match msg {
        PublicMessage::PrGetAutomatoList => Ok(ServerResponse::SrAutomatos(
            data.config.automato_ids.clone(),
        )),
        PublicMessage::PrGetSerialPortList => {
            let ports = available_ports()?
                .iter()
                .map(|s| get_port_info(&s))
                .collect();

            Ok(ServerResponse::SrSerialPorts(ports))
        }
        PublicMessage::PrAutomatoMsg(am) => {
            let mut mb = am::Msgbuf {
                buf: [0; am::RH_RF95_MAX_MESSAGE_LEN],
            };

            println!("sending automatomsg: {:?}", am);

            let mut retmsg = mb.clone();

            unsafe {
                mb.payload = am::Payload::from(am.message);
                let mut mp = data.port.lock()?;
                match mp.as_mut() {
                    Some(ref mut port) => {
                        am::write_message(port.as_mut(), &mb, &am.id)?;

                        // let mut fromid: u8 = 0;
                        // set to more than the hardcoded RHMesh timeout, which is 4000ms
                        port.set_timeout(Duration::from_millis(4420))?;

                        match am::read_message(port.as_mut(), &mut retmsg) {
                            Ok(fromid) => {
                                println!("reply from: {:?}", fromid);
                                // for i in 0..retmsg.buf.len() {
                                //     let c = retmsg.buf[i];
                                //     println!("{} - {}", c, c as char);
                                // }
                                am::print_payload(&retmsg.payload);

                                let rm = AutomatoMsg {
                                    id: fromid,
                                    message: am::PayloadEnum::from(retmsg.payload),
                                };
                                Ok(ServerResponse::SrAutomatoMsg(rm))
                            }
                            Err(e) => {
                                println!("read_message err: {:?}", e);
                                let se = serial_error::Error::from(e);
                                Ok(ServerResponse::SrSerialError(se))
                                // content: serde_json::Value::Null,
                            }
                        }
                    }
                    None => Ok(ServerResponse::SrGenericError("no serial port".to_string())),
                }
            }
        }
    }
}
