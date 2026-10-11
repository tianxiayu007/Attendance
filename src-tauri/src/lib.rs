mod attendance;
mod import_convert;
mod util;
use std::path::Path;

#[tauri::command]
fn import_convert(file: &str) -> String {
    let src_path = Path::new(file);
    return match import_convert::handle_import(src_path) {
        Ok(_) => format!("已生成 OA 导入文件：{}", output_file_name(src_path, "_oa.xlsx")),
        Err(e) => format!("导入失败：{}", e),
    };
}

#[tauri::command]
fn attendance_convert(file: &str) -> String {
    let src_path = Path::new(file);
    return match attendance::handle_attendance(src_path) {
        Ok(_) => format!(
            "已生成考勤报表：{}",
            output_file_name(src_path, "_report.xlsx")
        ),
        Err(e) => format!("生成报表失败：{}", e),
    };
}

/// 根据源文件推断输出文件名，仅用于界面提示。
fn output_file_name(src_path: &Path, suffix: &str) -> String {
    let stem = src_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "考勤文件".to_string());
    format!("{}{}", stem, suffix)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![import_convert, attendance_convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
