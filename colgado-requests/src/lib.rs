use errors::ColgadoRequestsError;
use models::{
    file_variables::FileVariables,
    info::{Bot, User},
};

use requests::send_msg_request;
use token_getter::token_flow;
pub mod errors;
pub mod models;
pub mod requests;
pub mod token_getter;
mod util;

pub const URL: &str = "wss://eventsub.wss.twitch.tv/ws";

pub async fn get_token() -> Result<(User, Bot, String), ColgadoRequestsError> {
    println!("Getting client id");
    let file_variables = util::open_file().await?;

    let FileVariables {
        client_id,
        redirect_urls,
        command,
    } = file_variables;

    println!("Getting token");
    let access_token = token_flow::get_token(&client_id, &redirect_urls).await?;
    println!("Token received");

    println!("Validating token and getting user_id");
    let (user_id, user_nick) = token_flow::validate_token(&access_token).await?;
    println!("Valid token");

    let user = User { user_id, user_nick };
    let bot = Bot {
        client_id,
        access_token,
    };
    Ok((user, bot, command))
}

pub async fn subscribe_to_wb(
    bot_info: &Bot,
    session_id: &str,
    broadcaster_user_id: &str,
    user_id: &str,
) -> Result<(), ColgadoRequestsError> {
    println!("Starting subscription");
    requests::websocket_subscription(bot_info, session_id, broadcaster_user_id, user_id).await?;
    println!("Suscription to websocket succesfull");
    Ok(())
}

pub async fn send_msg(
    bot_info: &Bot,
    broadcaster_user_id: &str,
    user_id: &str,
    message: &str,
) -> Result<(), ColgadoRequestsError> {
    send_msg_request(bot_info, broadcaster_user_id, user_id, message).await?;
    Ok(())
}
