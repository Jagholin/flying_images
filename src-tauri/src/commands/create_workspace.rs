use crate::state::{
    state::TauriState,
    workspace::{Workspace, WorkspaceUI},
};
use std::sync::Arc;

#[tauri::command]
pub async fn create_workspace(
    state: tauri::State<'_, Arc<TauriState>>,
    dir: &str,
    name: &str,
) -> Result<WorkspaceUI, String> {
    // *state.s.lock().unwrap() = "new string".into();
    // state.t.lock().unwrap().insert("key".into(), "value".into());
    println!("Creating the workspace, dir={}, name={}", dir, name);
    let mut state_lock = state.state.lock().await;
    let new_ws = Workspace::create_workspace(name, dir)?;
    let ws_data = new_ws.to_ui_data();
    state_lock.set_workspace(new_ws);
    println!("Workspace established, data {:?}", ws_data);

    Ok(ws_data)
}
