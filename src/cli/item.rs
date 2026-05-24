use clap::{Args};

#[derive(Args, Debug)]
pub struct ItemArgs {
    ///映射后的ID
    #[arg(long, short, required = true)]
    pub id: u64,
}
