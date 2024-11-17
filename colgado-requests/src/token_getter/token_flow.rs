use super::port_actor::ListenerHandle;
use crate::errors::TokenError;
use crate::util;

use rand::distributions::{Alphanumeric, DistString};
use reqwest::header;
use std::{collections::HashMap, ops::Deref};
use tokio::net::TcpListener;

pub async fn get_token<T>(client_id: &str, redirect_urls: &[T]) -> Result<String, TokenError>
where
    T: Deref<Target = str>,
{
    let mut redirect_url: Result<&str, TokenError> = Err(TokenError::InvalidToken);
    let mut listener: Result<TcpListener, TokenError> = Err(TokenError::InvalidToken);
    for url in redirect_urls {
        listener = TcpListener::bind(util::tcp_addres(url).await?)
            .await
            .map_err(|err| TokenError::IoError { err });
        if listener.is_ok() {
            redirect_url = Ok(url);
            break;
        }
    }
    let (listener, redirect_url) = (listener?, redirect_url?);

    let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let auth_url = get_authorization_url(client_id, &state, redirect_url);
    open::that(auth_url)?;
    let hash_map = listen_port(listener).await?;
    let token: &[String] = match hash_map.get("access_token") {
        Some(token) => token,
        None => return Err(TokenError::InvalidToken),
    };
    let token: &str = match token.first() {
        Some(token) => token,
        None => return Err(TokenError::InvalidToken),
    };
    let is_state = match hash_map.get("state") {
        Some(state_from_hashmap) => state_from_hashmap.contains(&state),
        None => false,
    };
    if !is_state {
        return Err(TokenError::DifferentStates);
    }
    Ok(token.to_owned())
}

/// Get the auth url of twitch to get the access token
///
fn get_authorization_url(client_id: &str, state: &str, redirect_url: &str) -> String {
    const RESPONSE_TYPE: &str = "token";
    const SCOPE: &str = "user:read:chat+user:write:chat";

    let params: [(&str, &str); 5] = [
        ("client_id", client_id),
        ("redirect_uri", redirect_url),
        ("response_type", RESPONSE_TYPE),
        ("scope", SCOPE),
        ("state", state),
    ];
    let mut url = "https://id.twitch.tv/oauth2/authorize?".to_owned();
    for param in params {
        url = url + "&" + param.0 + "=" + param.1;
    }
    url
}

async fn listen_port(listener: TcpListener) -> Result<HashMap<String, Vec<String>>, TokenError> {
    let mut listener_handle = ListenerHandle::new(listener);

    if let Some(token) = listener_handle.receive().await {
        listener_handle.kill_actor();
        return Ok(token);
    }

    listener_handle.kill_actor();
    Err(TokenError::TokenNotReceived)
}

pub async fn validate_token(token: &str) -> Result<(String, String), TokenError> {
    let token = "OAuth ".to_owned() + token;
    let response = reqwest::Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .header(header::AUTHORIZATION, token)
        .send()
        .await;
    let response: serde_json::Value = match response {
        Ok(response) if response.status() == 200 => response.json().await?,
        Ok(_) => return Err(TokenError::InvalidToken),
        Err(_) => return Err(TokenError::DeserializingError),
    };

    let user_id = response["user_id"].as_str();
    let user_nick = response["login"].as_str();

    let (user_id, user_nick) = match (user_id, user_nick) {
        (Some(user_id), Some(user_nick)) => (user_id.to_owned(), user_nick.to_owned()),
        _ => return Err(TokenError::DeserializingError),
    };

    Ok((user_id, user_nick))
}
