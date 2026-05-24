use clap::{Args, Subcommand};

///公共的属性
#[derive(Args, Debug)]
pub struct CommonArgs {
    ///名字
    #[arg(short, long, required = true)]
    pub name: String,
    ///注释
    #[arg(short = 't', long)]
    pub note: Option<String>, 
}

///需要创建的类型
#[derive(Subcommand, Debug)]
pub enum NewCommand {
    ///列表
    #[command(name = "l")]
    List(ListArgs),
    ///项
    #[command(name = "i", subcommand)]
    Item(ItemCommand)
}

///列表的属性
#[derive(Args, Debug)]
pub struct ListArgs {
    #[command(flatten)]
    pub comm: CommonArgs,
}

///项的公共属性
#[derive(Args, Debug)]
pub struct ListCommonArgs {
    ///要添加到的列表的id
    #[arg(short, long, required = true)]
    pub list: u64,
    ///截止时间
    #[arg(short, long = "ddl")]
    pub dead_line: Option<i64>,
    ///开始时间
    #[arg(short, long = "start")]
    pub start_time: Option<i64>,
    ///优先级
    #[arg(short, long = "pr")]
    pub priority: Option<u8>,
}

///项
#[derive(Debug, Subcommand)]
pub enum ItemCommand {
    #[command(name = "b")]
    Basic(BasicArgs),
    #[command(name = "o")]
    Open(OpenArgs)
}

///基础的属性
#[derive(Args, Debug)]
pub struct BasicArgs {
    #[command(flatten)]
    pub comm: CommonArgs,
    #[command(flatten)]
    pub list_comm: ListCommonArgs
}

///打开类型的属性
#[derive(Args, Debug)]
pub struct OpenArgs {
    #[command(flatten)]
    pub comm: CommonArgs,
    #[command(flatten)]
    pub list_comm: ListCommonArgs,
    ///待打开的路径
    #[arg(short, long, required = true)]
    pub path: Vec<String>,
    ///用该软件打开
    #[arg(short, long, required = true, num_args = 1..)]
    pub exe: String
}