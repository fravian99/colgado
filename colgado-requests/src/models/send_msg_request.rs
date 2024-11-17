use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SendMsgRequest<'a> {
    pub broadcaster_id: &'a str,
    pub sender_id: &'a str,
    pub message: String,
    #[serde(
        rename = "reply_parent_message_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to: Option<&'a str>,
}

impl<'a> SendMsgRequest<'a> {
    pub fn new(broadcaster_id: &'a str, sender_id: &'a str, message: &'a str) -> Self {
        let message = format!("[Colgado]: {}", message);
        Self {
            broadcaster_id,
            sender_id,
            message,
            reply_to: None,
        }
    }

    pub fn new_reply(
        broadcaster_id: &'a str,
        sender_id: &'a str,
        message: &'a str,
        reply_to: &'a str,
    ) -> Self {
        let mut send_msg_request = Self::new(broadcaster_id, sender_id, message);
        send_msg_request.reply_to = Some(reply_to);
        send_msg_request
    }
}
