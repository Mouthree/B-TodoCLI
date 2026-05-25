use clap::{Args, Subcommand};

///公共的属性
#[derive(Args, Debug)]
pub struct CommonChangeArgs {
    ///名字
    #[arg(short, long)]
    pub name: Option<String>,
    ///注释
    #[arg(short = 't', long)]
    pub note: Option<String>, 
    ///优先级
    #[arg(short, long)]
    pub priority: Option<u8>
}

///需要创建的类型
#[derive(Subcommand, Debug)]
pub enum ChangeCommand {
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
    pub comm: CommonChangeArgs,
    ///最近打开时间
    pub last_open: Option<i64>,
    ///完成情况
    pub current_state: Option<f64>,
    
}

///项的公共属性
#[derive(Args, Debug)]
pub struct ListCommonChangeArgs {
    ///添加到的列表的id
    #[arg(short, long)]
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
    pub comm: CommonChangeArgs,
    #[command(flatten)]
    pub list_comm: ListCommonChangeArgs
}

///打开类型的属性
#[derive(Args, Debug)]
pub struct OpenArgs {
    #[command(flatten)]
    pub comm: CommonChangeArgs,
    #[command(flatten)]
    pub list_comm: ListCommonChangeArgs,
    ///待打开路径
    #[arg(short, long)]
    pub open_file: Option<Vec<String>>,
    ///所使用软件
    #[arg(short, long)]
    pub by_exe: Option<String>
}
