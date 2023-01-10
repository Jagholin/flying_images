use reqwest::{Client, Method, Request, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtStationTokenResponse {
    public_csrf_token: String,
}

#[derive(Debug)]
pub struct ArtstationCsrf {
    public: String,
    private: String,
}

#[derive(Debug, Deserialize)]
pub struct ArtStationSearchResponseIcons {
    video: bool,
    image: bool,
    model3d: bool,
    marmoset: bool,
    pano: bool,
    multiple_images: bool,
    video_clip: bool,
}

#[derive(Debug, Deserialize)]
pub struct ArtStationSearchResponseUser {
    id: u32,
    username: String,
    full_name: String,
    is_staff: bool,
    pro_member: bool,
    is_plus_member: bool,
    is_studio_account: bool,
    is_school_account: bool,
}

#[derive(Debug, Deserialize)]
pub struct ArtStationSearchResponseItem {
    id: u32,
    hash_id: String,
    url: String,
    hide_as_adult: bool,
    title: String,
    icons: ArtStationSearchResponseIcons,
    user: ArtStationSearchResponseUser,
}

#[derive(Debug, Deserialize)]
pub struct ArtStationSearchResponse {
    total_count: u32,
    data: Vec<ArtStationSearchResponseItem>,
}

pub async fn request_csrf_token(cl: &Client) -> Result<ArtStationTokenResponse, String> {
    let res = cl
        .post("https:///www.artstation.com/api/v2/csrf_protection/token.json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let headers = res.headers();
    println!(
        "set cookie statement found: {}",
        headers.get("set-cookie").unwrap().to_str().unwrap()
    );

    let res = res
        .json::<ArtStationTokenResponse>()
        .await
        .map_err(|e| e.to_string())?;

    println!("get token response decoded: {:?}", res);
    Ok(res)
}
