use clap::{Args, Subcommand};

///公共属性（列表和项通用）
#[derive(Args, Debug)]
pub struct CommonChangeArgs {
    ///待修改的 id
    #[arg(short, long, required = true)]
    pub id: u64,
    ///名字
    #[arg(short, long)]
    pub name: Option<String>,
    ///注释
    #[arg(short = 't', long)]
    pub note: Option<String>,
    ///优先级
    #[arg(long = "pri")]
    pub priority: Option<u8>,
}

///修改列表或项
#[derive(Subcommand, Debug)]
pub enum ChangeCommand {
    ///列表
    #[command(name = "l")]
    List(ListArgs),
    ///项
    #[command(name = "i", subcommand)]
    Item(ItemCommand),
}

///修改列表
#[derive(Args, Debug)]
pub struct ListArgs {
    #[command(flatten)]
    pub comm: CommonChangeArgs,
    ///最近打开时间
    #[arg(short, long)]
    pub last_open: Option<i64>,
    ///完成情况
    #[arg(short, long)]
    pub current_state: Option<f64>,
}

///修改项
#[derive(Debug, Subcommand)]
pub enum ItemCommand {
    ///基本项
    #[command(name = "b")]
    Basic(BasicArgs),
    ///打开项
    #[command(name = "o")]
    Open(OpenArgs),
}

///修改基本项
#[derive(Args, Debug)]
pub struct BasicArgs {
    #[command(flatten)]
    pub comm: CommonChangeArgs,
    ///截止时间
    #[arg(short, long = "ddl")]
    pub dead_line: Option<i64>,
    ///开始时间
    #[arg(short, long = "start")]
    pub start_time: Option<i64>,
}

///修改打开项
#[derive(Args, Debug)]
pub struct OpenArgs {
    #[command(flatten)]
    pub comm: CommonChangeArgs,
    ///截止时间
    #[arg(short, long = "ddl")]
    pub dead_line: Option<i64>,
    ///开始时间
    #[arg(short, long = "start")]
    pub start_time: Option<i64>,
    ///待打开路径
    #[arg(short, long)]
    pub open_file: Option<Vec<String>>,
    ///所使用软件
    #[arg(short, long)]
    pub by_exe: Option<String>,
}
