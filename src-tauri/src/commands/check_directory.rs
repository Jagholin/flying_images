#[tauri::command]
pub fn check_directory(dir: &str) -> bool {
    let path = std::path::Path::new(dir);
    path.is_dir()
}
