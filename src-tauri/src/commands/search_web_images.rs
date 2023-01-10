use crate::state::state::TauriState; 
use crate::request::artstation_api::{ArtStationTokenResponse, request_csrf_token};
use crate::request::da_api::request_deviantart_auth;

#[tauri::command]
async fn search_web_images(state: tauri::State<'_, TauriState>, query: &str) -> Result<(), String> {
    Ok(())    
}

#[tauri::command]
pub async fn get_csrf_token(state: tauri::State<'_, TauriState>) -> Result<ArtStationTokenResponse, String> {
    request_csrf_token(&state.reqwest_client).await
}

#[tauri::command]
pub async fn test_da_request(state: tauri::State<'_, TauriState>) -> Result<(), String> {
    let res = request_deviantart_auth(&state.reqwest_client).await?;
    println!("Response got: {:?}", res);
    Ok(())
}
