mod attendance;
mod import_convert;
mod util;
use std::path::Path;

#[tauri::command]
fn import_convert(file: &str) -> String {
    let src_path = Path::new(file);
    return match import_convert::handle_import(src_path) {
        Ok(_) => "转换成功".to_string(),
        Err(e) => format!("{}", e),
    };
}

#[tauri::command]
fn attendance_convert(file: &str) -> String {
    let src_path = Path::new(file);
    return match attendance::handle_attendance(src_path) {
        Ok(_) => "转换成功".to_string(),
        Err(e) => format!("{}", e),
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![import_convert, attendance_convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
