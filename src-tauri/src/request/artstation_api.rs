use reqwest::{Client, Method, Request, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtStationTokenResponse {
    public_csrf_token: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ArtStationCsrf {
    public: String,
    private: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtStationSearchResponseIcons {
    video: bool,
    image: bool,
    model3d: bool,
    marmoset: bool,
    pano: bool,
    multiple_images: bool,
    video_clip: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtStationSearchResponseUser {
    pub id: u32,
    pub username: String,
    pub full_name: String,
    is_staff: bool,
    pro_member: bool,
    is_plus_member: bool,
    is_studio_account: bool,
    is_school_account: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtStationSearchResponseItem {
    id: u32,
    pub hash_id: String,
    pub url: String,
    pub hide_as_adult: bool,
    pub title: String,
    icons: ArtStationSearchResponseIcons,
    pub user: ArtStationSearchResponseUser,
}

#[derive(Debug, Deserialize)]
pub struct ArtStationSearchResponse {
    pub total_count: u32,
    pub data: Vec<ArtStationSearchResponseItem>,
}

#[derive(Debug, Serialize)]
struct ArtStationSearchRequest {
    additional_fields: Vec<u32>, // datatype unknown, just some array
    filters: Vec<u32>,           // datatype unknown, just some array
    page: u32,
    per_page: u32,
    pro_first: String, // just "1" or "0"
    query: String,
    sorting: String,
}

pub async fn request_csrf_token(cl: &Client) -> Result<ArtStationCsrf, String> {
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

    let header_cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    let (header_cookie, _) = header_cookie.split_once(";").unwrap();
    let (_, header_cookie) = header_cookie.split_once("=").unwrap();
    let header_cookie = header_cookie.to_string();

    let res = res
        .json::<ArtStationTokenResponse>()
        .await
        .map_err(|e| e.to_string())?;

    println!("get token response decoded: {:?}", res);
    Ok(ArtStationCsrf {
        public: res.public_csrf_token,
        private: header_cookie,
    })
}

pub async fn search_images(
    cl: &Client,
    auth: &ArtStationCsrf,
    query: String,
    page: u32,
) -> Result<ArtStationSearchResponse, String> {
    println!("query is {query}");
    let res = cl
        .post("https://www.artstation.com/api/v2/search/projects.json")
        .header("PUBLIC-CSRF-TOKEN", &auth.public)
        .header("Cookie", format!("PRIVATE-CSRF-TOKEN={}", auth.private));
    let request_data = ArtStationSearchRequest {
        additional_fields: vec![],
        filters: vec![],
        page: page,
        per_page: 50,
        pro_first: "1".to_string(),
        query: query,
        sorting: "relevance".to_string(),
    };
    let request_string = serde_json::to_string_pretty(&request_data).unwrap();
    println!("request string is {}", request_string);
    let res = res
        .header("Content-Type", "application/json")
        .body(request_string)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let res = res.text().await.unwrap();
    println!("as image search response {}", res);
    if res.contains("{") {
        // assume normal JSON response
        let res =
            serde_json::from_str::<ArtStationSearchResponse>(&res).map_err(|e| e.to_string())?;
        return Ok(res);
    }
    Err(res)
}
