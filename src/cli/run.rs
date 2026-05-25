use clap::{Args};

#[derive(Args, Debug)]
pub struct RunArgs {
    ///待运行id
    #[arg(long, short, required = true, num_args = 1..)]
    pub id: Vec<u64>,
    ///运行列表还是项
    #[arg(long, short, required = true)]
    pub run_type: String
}