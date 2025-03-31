pub mod actors;
pub mod errors;
pub mod models;
mod word;

use std::sync::Arc;

use crate::actors::game_actor::TwitchGameHandle;
use crate::actors::message_actor::TwitchMessageHandle;

use errors::ColgadoLogicError;
use models::handles::Handles;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use trequests::{errors::TRequestsError, open_file, URL};

const FILE: &str = "env.toml";

pub async fn init_flow() -> Result<(Handles, Arc<[JoinHandle<()>]>, Box<str>), ColgadoLogicError> {
    let file_variables = open_file(FILE)
        .await
        .map_err(|err| TRequestsError::VarError { err })?;
    let (user, bot_info) = trequests::get_token(&file_variables).await?;
    let command = file_variables.command;

    let (twitch_game_handle, twitch_game_task) =
        TwitchGameHandle::new_and_joinhandle(user, bot_info, command.clone());

    let (ws_stream, _) = connect_async(URL).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (twitch_message_handle, twitch_message_task) =
        TwitchMessageHandle::new_and_joinhandle(ws_stream, twitch_game_handle.clone());

    let handles = Handles {
        message_handle: twitch_message_handle,
        game_handle: twitch_game_handle,
    };
    let tasks = vec![twitch_game_task, twitch_message_task];
    let command: Box<str> = command.into();
    Ok((handles, tasks.into(), command))
}
