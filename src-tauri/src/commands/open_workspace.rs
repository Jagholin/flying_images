use crate::state::{
    state::TauriState,
    workspace::{Workspace, WorkspaceUI},
};
use std::sync::Arc;

#[tauri::command]
pub async fn open_workspace(
    state: tauri::State<'_, Arc<TauriState>>,
    dir: &str,
) -> Result<WorkspaceUI, String> {
    let mut state_lock = state.state.lock().await;
    let ws = Workspace::load_workspace(dir)?;
    let ws_data = ws.to_ui_data();

    state_lock.set_workspace(ws);

    Ok(ws_data)
}
