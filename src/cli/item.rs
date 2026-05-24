use clap::{Args};

#[derive(Args, Debug)]
pub struct ItemArgs {
    ///映射后的ID
    #[arg(long, short)]
    pub id: u64,
}
