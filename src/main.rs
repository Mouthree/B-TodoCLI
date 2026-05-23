use B_TodoCLI::config::AppConfig;
use tracing::info;

fn main() {
    tracing_subscriber::fmt().pretty().init();
    info!("start");
    println!("{:?}", AppConfig::new());
    
}
