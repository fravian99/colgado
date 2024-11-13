use crate::actors::{game_actor::TwitchGameHandle, message_actor::TwitchMessageHandle};

#[derive(Debug, Clone)]
pub struct Handles {
    pub message_handle: TwitchMessageHandle,
    pub game_handle: TwitchGameHandle,
}
