//!配置文件管理
#![allow(dead_code)]
use std::{fs::{self}, path::{Path}};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::info;



///主配置
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    test: String
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        info!("开始加载配置");
        let proj_dirs = ProjectDirs::from("com", "MtrM", "mrTodo")
            .context("无法确定用户配置目录")?;
        let config_dir = proj_dirs.config_dir();          // &Path
        let config_file = config_dir.join("config.toml"); // 配置文件完整路径

        info!("配置目录: {}", config_dir.display());

        if config_file.is_file() {
            info!("找到已有配置文件，开始读取");
            let contents = std::fs::read_to_string(&config_file)
                .with_context(|| format!("无法读取配置文件 {}", config_file.display()))?;
            let config: AppConfig = toml::from_str(&contents)
                .context("配置文件格式错误")?;
            Ok(config)
        } else {
            info!("未找到配置文件，创建默认配置");
            let default_config = AppConfig {
                test: String::from("yeah")
            };
            let toml_string = toml::to_string_pretty(&default_config)?;
            if let Some(parent) = config_file.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("无法创建配置目录 {}", parent.display()))?;
            }
            fs::write(&config_file, &toml_string)
                .with_context(|| format!("无法写入配置文件 {}", config_file.display()))?;

            info!("默认配置文件已创建: {}", config_file.display());
            Ok(default_config)
        }
    }
}