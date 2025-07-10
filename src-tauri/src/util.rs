use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use serde::Deserialize;
use std::path::PathBuf;
use std::env;
use std::ffi::OsStr;

#[derive(Debug, Deserialize)]
pub struct Departments {
    #[serde(flatten)] // 展开所有顶层属性
    pub departments: IndexMap<String, IndexMap<String, String>>,
}

pub fn read_json_config() -> Result<Departments> {
    let config_path = get_current_dir().join("baobiao.json");
    if !config_path.is_file() {
        return Err(anyhow!("配置文件baobiao.json不存在"));
    }

    let file = std::fs::File::open(config_path)?;
    let reader = std::io::BufReader::new(file);
    let config: Departments = serde_json::from_reader(reader)?;
    Ok(config)
}

fn get_current_dir() -> PathBuf {
    let exe_path = env::current_exe().expect("无法获取可执行文件路径");

    if cfg!(target_os = "macos") {
        let mut path = exe_path.clone();
        while path.pop() {
            if path.extension() == Some(OsStr::new("app")) && path.join("Contents/MacOS").exists() {
                return path.parent().unwrap().to_path_buf();
            }
        }
    }

    exe_path.parent().unwrap().to_path_buf()
}
