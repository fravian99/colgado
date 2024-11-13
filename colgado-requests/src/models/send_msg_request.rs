use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SendMsgRequest<'a> {
    pub broadcaster_id: &'a str,
    pub sender_id: &'a str,
    pub message: &'a str,
}
