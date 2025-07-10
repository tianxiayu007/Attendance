use crate::util::read_json_config;
use anyhow::{Result, anyhow};
use std::path::Path;

pub fn handle_import(src_path: &Path) -> Result<(), anyhow::Error> {
    let json_config = read_json_config().map_err(|e| anyhow!("读取配置文件失败: {}", e))?;

    if !src_path.is_file() {
        return Err(anyhow!("源文件不存在"));
    }

    let ext = src_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();

    if ext != "xls" && ext != "xlsx" {
        return Err(anyhow!("仅支持 .xls/.xlsx 文件"));
    }
    println!("{:?}", json_config.departments);

    Ok(())
}
