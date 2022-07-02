use automato::automatomsg as am;
use serde_derive::{Deserialize, Serialize};

// -------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAutomato {
    pub id: i64,
    pub info: Option<am::RemoteInfo>,
}
