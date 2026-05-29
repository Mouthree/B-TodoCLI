use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub struct RunArgs {
    ///待运行id
    #[arg(long, short, required = true, num_args = 1..)]
    pub id: Vec<u64>,
    ///运行列表还是项
    #[arg(long, short, required = true)]
    pub run_type: RunType
}

///运行的类型
#[derive(Debug, ValueEnum, Clone)]
pub enum RunType {
    ///运行整个列表中所有可以运行的
    List,
    ///运行单独的一个项
    Item
}