use clap::{Args};

#[derive(Args, Debug)]
pub struct CdArgs {
    ///需要固定的命令
    #[arg(long, short, required = true)]
    pub comm: String,
}