use core::str;

use crate::models::{
    info::Bot, send_msg_request::SendMsgRequest, wb_subscription::EventSubRequestListenerBuilder,
};

pub async fn websocket_subscription(
    bot_info: &Bot,
    session_id: &str,
    broadcaster_user_id: &str,
    user_id: &str,
) -> Result<(), reqwest::Error> {
    let body = EventSubRequestListenerBuilder::new()
        .type_param("channel.chat.message")
        .version("1")
        .broadcaster_user_id(broadcaster_user_id)
        .user_id(user_id)
        .method("websocket")
        .session_id(session_id)
        .build();

    let request = reqwest::Client::new()
        .post("https://api.twitch.tv/helix/eventsub/subscriptions")
        .header("Client-Id", &bot_info.client_id)
        .bearer_auth(&bot_info.access_token)
        .json(&body);

    let response = request.send().await?;
    response.error_for_status()?;
    Ok(())
}

pub async fn send_msg_reqwest(
    bot_info: &Bot,
    broadcaster_id: &str,
    user_id: &str,
    message: &str,
) -> Result<(), reqwest::Error> {
    let body = SendMsgRequest {
        broadcaster_id,
        sender_id: user_id,
        message,
    };

    let request = reqwest::Client::new()
        .post("https://api.twitch.tv/helix/chat/messages")
        .header("Client-Id", &bot_info.client_id)
        .bearer_auth(&bot_info.access_token)
        .json(&body);

    let response = request.send().await?;
    response.error_for_status()?;
    Ok(())
}
