pub mod actors;
pub mod errors;
pub mod models;
mod word;

use std::sync::Arc;

use crate::actors::game_actor::TwitchGameHandle;
use crate::actors::message_actor::TwitchMessageHandle;

use colgado_requests::URL;
use errors::ColgadoLogicError;
use models::handles::Handles;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;

pub async fn init_flow() -> Result<(Handles, Arc<[JoinHandle<()>]>), ColgadoLogicError> {
    let (user, bot_info, command) = colgado_requests::get_token().await?;

    let (twitch_game_handle, twitch_game_task) =
        TwitchGameHandle::new_and_joinhandle(user, bot_info, command);

    let (ws_stream, _) = connect_async(URL).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (twitch_message_handle, twitch_message_task) =
        TwitchMessageHandle::new_and_joinhandle(ws_stream, twitch_game_handle.clone());

    let handles = Handles {
        message_handle: twitch_message_handle,
        game_handle: twitch_game_handle,
    };
    let tasks = vec![twitch_game_task, twitch_message_task];
    Ok((handles, tasks.into()))
}
