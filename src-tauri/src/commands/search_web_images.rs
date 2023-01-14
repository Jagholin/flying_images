use serde::Serialize;

use crate::request::artstation_api::{
    request_csrf_token, search_images as artstation_search_images, ArtStationCsrf,
    ArtStationSearchResponse, ArtStationSearchResponseItem, ArtStationTokenResponse,
};
use crate::request::da_api::{
    request_deviantart_auth, search_images as da_search_images, DeviantArtAuth, DeviantArtError,
    DeviantArtSearchResponse, DeviantArtSearchResponseItem, ErrorOr,
};
use crate::state::state::TauriState;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub enum ImageSearchAPIData {
    DeviantArt(DeviantArtSearchResponseItem),
    ArtStation(ArtStationSearchResponseItem),
}

#[derive(Debug, Serialize)]
pub struct ImageSearchResult {
    img_url: String,
    title: String,
    author_id: String,
    author_name: String,
    preview_url: String,
    id: String,
    mature_content: bool,

    api_data: ImageSearchAPIData,
}

async fn da_reauth(state: Arc<TauriState>) -> Result<DeviantArtAuth, String> {
    match request_deviantart_auth(&state.reqwest_client).await? {
        ErrorOr::Error(er) => Err(er.error_description),
        ErrorOr::Success(su) => Ok(su),
    }
}

async fn search_da_images(
    state: Arc<TauriState>,
    query: String,
) -> Result<DeviantArtSearchResponse, String> {
    println!("start searching da images...");
    let mut auth = state.da_auth.lock().await;
    let auth = match &*auth {
        Some(a) => a.to_owned(),
        None => {
            let new_auth = da_reauth(state.clone()).await?;
            *auth = Some(new_auth.clone());
            new_auth
        }
    };
    let result = match da_search_images(&state.reqwest_client, query.to_string(), &auth).await? {
        ErrorOr::Error(er) => {
            // try to reauth
            let new_auth = da_reauth(state.clone()).await?;
            *state.da_auth.blocking_lock() = Some(new_auth.clone());
            let response =
                da_search_images(&state.reqwest_client, query.to_string(), &new_auth).await?;
            match response {
                ErrorOr::Error(err) => Err(err.error_description),
                ErrorOr::Success(su) => Ok(su),
            }
        }
        ErrorOr::Success(su) => Ok(su),
    };
    println!("end searching da images...");
    result
}

async fn search_artstation_images(
    state: Arc<TauriState>,
    query: String,
) -> Result<ArtStationSearchResponse, String> {
    println!("start searching artstation images...");
    let mut csrf = state.artstation_tokens.lock().await;
    let csrf = match &*csrf {
        Some(c) => c.to_owned(),
        None => {
            let new_csrf = request_csrf_token(&state.reqwest_client).await?;
            *csrf = Some(new_csrf.clone());
            new_csrf
        }
    };
    let result = artstation_search_images(&state.reqwest_client, &csrf, query.to_string(), 1).await;
    println!("end searching artstation images");
    result
    // search_images(state.reqwest_client, )
}

#[tauri::command]
pub async fn search_web_images(
    state: tauri::State<'_, Arc<TauriState>>,
    wnd: tauri::Window,
    query: &str,
) -> Result<Vec<ImageSearchResult>, String> {
    let da_handle =
        tauri::async_runtime::spawn(search_da_images((*state).clone(), query.to_string()));
    let as_handle = tauri::async_runtime::spawn(search_artstation_images(
        (*state).clone(),
        query.to_string(),
    ));
    let res_da = da_handle.await.map_err(|e| {
        println!("error in res_da {:?}", e);
        e.to_string()
    })??;
    let res_as = as_handle.await.map_err(|e| {
        println!("error in res_as {:?}", e);
        e.to_string()
    })??;

    let mut result = vec![];
    for dresp in res_da.results {
        let (Some(content), Some(title), Some(author), Some(preview)) = (&dresp.content, &dresp.title, &dresp.author, &dresp.preview) else {
            println!("content, title, preview or author missing in {:?}", dresp);
            continue;
        };
        result.push(ImageSearchResult {
            id: dresp.deviationid.clone(),
            mature_content: dresp.is_mature.unwrap_or(false),
            img_url: content.src.clone(),
            title: title.clone(),
            author_id: author.userid.clone(),
            author_name: author.username.clone(),
            preview_url: preview.src.clone(),
            api_data: ImageSearchAPIData::DeviantArt(dresp),
        });
    }
    for aresp in res_as.data {
        result.push(ImageSearchResult {
            id: aresp.hash_id.clone(),
            mature_content: aresp.hide_as_adult,
            img_url: aresp.url.clone(),
            title: aresp.title.clone(),
            author_id: aresp.user.id.to_string(),
            author_name: aresp.user.full_name.clone(),
            preview_url: aresp.smaller_square_cover_url.clone(),
            api_data: ImageSearchAPIData::ArtStation(aresp),
        })
    }
    Ok(result)
}

#[tauri::command]
pub async fn get_csrf_token(
    state: tauri::State<'_, Arc<TauriState>>,
) -> Result<ArtStationCsrf, String> {
    request_csrf_token(&state.reqwest_client).await
}

#[tauri::command]
pub async fn test_da_request(state: tauri::State<'_, Arc<TauriState>>) -> Result<(), String> {
    let res = request_deviantart_auth(&state.reqwest_client).await?;
    println!("Response got: {:?}", res);
    Ok(())
}
