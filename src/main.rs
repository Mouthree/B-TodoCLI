use std::env;

use B_TodoCLI::{start};
use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    let raw_args: Vec<String> = env::args().collect();
    println!("{:?}", raw_args);
    // info!("start");
    // start()?;
    Ok(())
}
