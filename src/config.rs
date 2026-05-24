//!配置文件管理
#![allow(dead_code)]
use std::{fs::{self}, path::PathBuf};

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
        let config_file = Self::set_config_file(None)?;

        if config_file.is_file() {
            Self::get_local_config(&config_file)
        } else {
            info!("未找到配置文件，创建默认配置");
            let default_config = AppConfig::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            Self::create_default_config_path(&config_file)?;
            Self::write_default_config_file(&config_file, toml_string)?;
            Ok(default_config)
        }
    }

    ///设置本地config路径
    fn set_config_file(path: Option<PathBuf>) -> Result<PathBuf> {
        let path = if let Some(path) = path {
            path.join("config.toml")
        } else {
            let dirs = ProjectDirs::from("com", "MtrM", "mrTodo")
                .context("无法确定用户配置目录")?;
            let path = dirs.config_dir().join("config.toml");
            path
        };
        info!("配置路径: {}", path.display());
        Ok(path)
    }

    ///获取本地数据
    fn get_local_config(config_file: &PathBuf) -> Result<AppConfig> {
        info!("读取已有配置文件");
        let contents = std::fs::read_to_string(config_file)
            .with_context(|| format!("无法读取配置文件 {}", config_file.display()))?;
        let config: AppConfig = toml::from_str(&contents)
            .context("配置文件格式错误")?;
        info!("读取成功");
        Ok(config)
    }

    ///创建本地文件路径
    fn create_default_config_path(config_file: &PathBuf) -> Result<()> {
        if let Some(path) = config_file.parent() {
            fs::create_dir_all(path)
                .with_context(|| format!("无法创建配置目录 {}", path.display()))?;
            info!("创建目录: {}", path.display());
        }
        Ok(())
    }

    ///写入默认配置文件
    fn write_default_config_file(config_file: &PathBuf, toml_string: String) -> Result<()> {
        fs::write(config_file, &toml_string)
            .with_context(|| format!("无法写入配置文件 {}", config_file.display()))?;
        info!("默认配置文件已创建: {}", config_file.display());
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { test: Default::default() }
    }
}

#[cfg(test)]
mod tests {
    
}