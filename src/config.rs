//!配置文件管理
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("序列化失败: {0}")]
    Serde(#[from] serde_json::Error)
}


///主配置
#[derive(Debug, Deserialize)]
struct AppConfig {
    log: LogConfig
}

///日志配置
#[derive(Debug, Deserialize)]
struct LogConfig {
    log_local: String
}

impl AppConfig {
    fn new(&self) -> Result<(), ConfigError> {
        let config = AppConfig {
            log: LogConfig { log_local: "log/".to_string() },
        };
        Ok(())
    }
}