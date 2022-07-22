use crate::config::Config;
// use crate::data::{};
use crate::messages::{PublicMessage, ServerResponse};
// use crate::sqldata;
use crate::data::{AutomatoMsg, ListAutomato, ServerData};
use log::info;
use std::error::Error;

// public json msgs don't require login.
pub fn public_interface(
    data: &ServerData,
    msg: PublicMessage,
) -> Result<ServerResponse, Box<dyn Error>> {
    info!("process_public_json, what={}", msg.what.as_str());
    match msg.what.as_str() {
        "GetAutomatoList" => Ok(ServerResponse {
            what: "automatos".to_string(),
            content: serde_json::to_value(data.config.automato_ids.clone())?,
        }),
        "GetAutomatoInfo" => {
            let msgdata = Option::ok_or(msg.data.as_ref(), "malformed json data")?;
            let am: AutomatoMsg = serde_json::from_value(msgdata.clone())?;

            Ok(ServerResponse {
                what: "unimplemented".to_string(),
                content: serde_json::Value::Null,
            })
        }
        wat => Err(Box::new(simple_error::SimpleError::new(format!(
            "invalid 'what' code:'{}'",
            wat
        )))),
    }
}
