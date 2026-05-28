use std::env;

use B_TodoCLI::{start};
use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    //启动调试
    tracing_subscriber::fmt().pretty().init();
    //获取启动参数
    let raw_args: Vec<String> = env::args().skip(1).collect();
    let start_items = if raw_args.is_empty() {None} else {Some(raw_args.join(" "))};
    info!("start");
    start(start_items).await?;
    Ok(())
}
