use crate::{
    errors::{GettingDataError, TokenError},
    models::file_variables::FileVariables,
};
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::lookup_host;
use url::Url;

pub fn get_params(request: &hyper::Request<hyper::body::Incoming>) -> HashMap<String, Vec<String>> {
    let uri = request.uri();
    let query = uri.query();
    let mut params: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(query_param) = query {
        let param = url::form_urlencoded::parse(query_param.as_bytes()).into_owned();
        param
            .collect::<Vec<(String, String)>>()
            .iter()
            .for_each(|element: &(String, String)| {
                let key = &element.0;
                let value = params.get_mut(key);
                match value {
                    Some(value) => {
                        value.push(element.1.clone());
                    }
                    None => {
                        let value = vec![element.1.clone()];
                        params.insert(key.clone(), value);
                    }
                };
            });
    }

    params
}

pub async fn tcp_addres(url: &str) -> Result<SocketAddr, TokenError> {
    let url = Url::parse(url)?;
    let host_and_port = match (url.host_str(), url.port()) {
        (Some(host), Some(port)) => host.to_owned() + ":" + &port.to_string(),
        _ => return Err(TokenError::InvalidToken),
    };
    let address = lookup_host(host_and_port).await?.next();
    let address = match address {
        Some(address) => address,
        None => {
            return Err(TokenError::InvalidToken);
        }
    };
    Ok(address)
}

pub async fn open_file() -> Result<FileVariables, GettingDataError> {
    const FILE: &str = "env.toml";
    let string = tokio::fs::read_to_string(FILE).await?;
    let file_variables = toml::from_str(&string)?;
    Ok(file_variables)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_tcp_address() {
        let address = tcp_addres("http://localhost:3000/probando-para-que-no-se-vea").await;
        if let Err(address) = &address {
            println!("Error: {}", address);
        }
        assert!(address.is_ok());
        if let Ok(address) = address {
            assert_eq!("127.0.0.1:3000", address.to_string());
        }
    }
}
