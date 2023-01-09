#[tauri::command]
pub fn check_directory(dir: &str) -> bool {
    println!("Checking the directory {}", dir);

    let path = std::path::Path::new(dir);
    path.is_dir()
}
