use B_TodoCLI::{cli, config::AppConfig, start};
use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    info!("start");
    start()?;
    Ok(())
}
