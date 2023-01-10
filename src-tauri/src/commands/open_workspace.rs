use crate::state::{state::TauriState, workspace::{Workspace, WorkspaceUI}};

#[tauri::command]
pub async fn open_workspace(state: tauri::State<'_, TauriState>, dir: &str) -> Result<WorkspaceUI, String> {
    let mut state_lock = state.state.lock().unwrap();
    let ws = Workspace::load_workspace(dir)?;
    let ws_data = ws.to_ui_data();

    state_lock.set_workspace(ws);

    Ok(ws_data)
}