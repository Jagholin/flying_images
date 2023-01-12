use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct DeviantArtAuth {
    access_token: String,
    // status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviantArtUser {
    pub userid: String,
    pub username: String,
    usericon: String,

    #[serde(rename = "type")]
    user_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviantArtFileDescriptor {
    pub src: String,
    height: u32,
    width: u32,
    transparency: bool,
    filesize: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviantArtSearchResponseItem {
    deviationid: String,
    pub url: Option<String>,
    pub title: Option<String>,
    is_mature: Option<bool>,
    is_downloadable: Option<bool>,
    download_filesize: Option<u32>,
    category: Option<String>,
    pub author: Option<DeviantArtUser>,
    published_time: Option<String>,
    preview: Option<DeviantArtFileDescriptor>,
    pub content: Option<DeviantArtFileDescriptor>,
    thumbs: Option<Vec<DeviantArtFileDescriptor>>,
}

#[derive(Debug, Deserialize)]
pub struct DeviantArtSearchResponse {
    pub has_more: bool,
    next_offset: u32,
    pub estimated_total: Option<u32>,
    pub results: Vec<DeviantArtSearchResponseItem>,
}

#[derive(Debug, Deserialize)]
pub struct DeviantArtError {
    pub error: String,
    pub error_description: String,
    pub status: String, // "error"
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
        .query(&[
            ("client_id", "22403"),
            ("client_secret", "75ffac46bb8830d19cd66ef78dd2d141"),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let res = res.text().await.map_err(|e| e.to_string())?;
    println!("response got: {}", &res);
    let res = serde_json::from_str::<ErrorOr<DeviantArtAuth>>(&res).map_err(|e| e.to_string())?;
    // let res = res.json::<ErrorOr<DeviantArtAuthResponse>>().await.map_err(|e| e.to_string())?;

    Ok(res)
}

pub async fn search_images(
    cl: &Client,
    q: String,
    auth: &DeviantArtAuth,
) -> Result<ErrorOr<DeviantArtSearchResponse>, String> {
    let res = cl
        .get("https://www.deviantart.com/api/v1/oauth2/browse/newest")
        .bearer_auth(&auth.access_token)
        .query(&[("mature_content", "true"), ("q", &q)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let res = res.text().await.unwrap();
    // println!("da response search_images got {}", res);

    serde_json::from_str::<ErrorOr<DeviantArtSearchResponse>>(&res)
        // .await
        .map_err(|e| e.to_string())
}
