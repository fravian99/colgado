use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct EventSubRequestListener<'a> {
    #[serde(rename = "type")]
    type_param: &'a str,
    version: &'a str,
    condition: Condition<'a>,
    transport: Transport<'a>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Condition<'a> {
    broadcaster_user_id: &'a str,
    user_id: &'a str,
}
#[derive(Clone, Deserialize, Serialize, Debug)]
struct Transport<'a> {
    method: &'a str,
    session_id: &'a str,
}

#[derive(Debug, Default)]
pub struct EventSubRequestListenerBuilder<'a> {
    type_param: &'a str,
    version: &'a str,
    broadcaster_user_id: &'a str,
    user_id: &'a str,
    method: &'a str,
    session_id: &'a str,
}

impl<'a> EventSubRequestListenerBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn type_param(mut self, type_param: &'a str) -> Self {
        self.type_param = type_param;
        self
    }

    pub fn version(mut self, version: &'a str) -> Self {
        self.version = version;
        self
    }

    pub fn broadcaster_user_id(mut self, broadcaster_user_id: &'a str) -> Self {
        self.broadcaster_user_id = broadcaster_user_id;
        self
    }

    pub fn user_id(mut self, user_id: &'a str) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn method(mut self, method: &'a str) -> Self {
        self.method = method;
        self
    }

    pub fn session_id(mut self, session_id: &'a str) -> Self {
        self.session_id = session_id;
        self
    }

    pub fn build(self) -> EventSubRequestListener<'a> {
        let condition = Condition {
            broadcaster_user_id: self.broadcaster_user_id,
            user_id: self.user_id,
        };

        let transport = Transport {
            method: self.method,
            session_id: self.session_id,
        };

        EventSubRequestListener {
            type_param: self.type_param,
            version: self.version,
            condition,
            transport,
        }
    }
}
