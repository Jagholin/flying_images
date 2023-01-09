use crate::state::state::ManagedState; 
use crate::request::image_search::{ArtStationTokenResponse, request_csrf_token};

#[tauri::command]
async fn search_web_images(state: tauri::State<'_, ManagedState>, query: &str) -> Result<(), String> {
    Ok(())    
}

#[tauri::command]
pub async fn get_csrf_token(state: tauri::State<'_, ManagedState>) -> Result<ArtStationTokenResponse, String> {
    request_csrf_token(&state.reqwest_client).await
}
