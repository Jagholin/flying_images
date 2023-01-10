use reqwest::{Client};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeviantArtAuth {
    access_token: String,
    // status: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviantArtError {
    error: String,
    error_description: String,
    status: String, // "error"
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ErrorOr<T> {
    Error(DeviantArtError),
    Success(T),
}

pub async fn request_deviantart_auth(cl: &Client) -> Result<ErrorOr<DeviantArtAuth>, String> {
    let res = cl
        .get("https://www.deviantart.com/oauth2/token")
        .query(&[("client_id", "22403"), ("client_secret", "75ffac46bb8830d19cd66ef78dd2d141"), ("grant_type", "client_credentials")])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let res = res.text().await.map_err(|e| e.to_string())?;
    println!("response got: {}", &res);
    let res = serde_json::from_str::<ErrorOr<DeviantArtAuth>>(&res).map_err(|e| e.to_string())?;
    // let res = res.json::<ErrorOr<DeviantArtAuthResponse>>().await.map_err(|e| e.to_string())?;

    Ok(res)
}
