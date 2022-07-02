use crate::config::Config;
// use crate::data::{};
use crate::messages::{PublicMessage, ServerResponse};
// use crate::sqldata;
use log::info;
use std::error::Error;

// public json msgs don't require login.
pub fn public_interface(
    config: &Config,
    msg: PublicMessage,
) -> Result<ServerResponse, Box<dyn Error>> {
    info!("process_public_json, what={}", msg.what.as_str());
    match msg.what.as_str() {
        "GetAutomatoList" => Ok(ServerResponse {
            what: "automatos".to_string(),
            content: serde_json::to_value(config.automato_ids.clone())?,
        }),
        "GetAutomatoDetail" => {
            let msgdata = Option::ok_or(msg.data.as_ref(), "malformed json data")?;
            let aid: i64 = serde_json::from_value(msgdata.clone())?;
            Ok(ServerResponse {
                what: "unimplemented".to_string(),
                content: serde_json::Value::Null,
            })

            //     let conn = sqldata::connection_open(config.orgauth_config.db.as_path())?;
            //     let project = sqldata::read_project_time(&conn, pid)?;

            //     if project.project.public {
            //         Ok(ServerResponse {
            //             what: "projecttime".to_string(),
            //             content: serde_json::to_value(project)?,
            //         })
            //     } else {
            //         Ok(ServerResponse {
            //             what: "projecttime-denied".to_string(),
            //             content: serde_json::Value::Null,
            //         })
            //     }
        }
        wat => Err(Box::new(simple_error::SimpleError::new(format!(
            "invalid 'what' code:'{}'",
            wat
        )))),
    }
}
