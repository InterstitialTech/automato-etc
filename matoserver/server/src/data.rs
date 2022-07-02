use automato::automatomsg as am;
use automato::automatomsg::Msgbuf;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// -------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAutomato {
    pub id: i64,
    pub info: Option<am::RemoteInfo>,
}
