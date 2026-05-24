use clap::{Parser, Subcommand};
use clap::{Args, ValueEnum};
use crate::cli::item::ItemArgs;
use crate::cli::new::NewCommand;
mod change;
mod del;
mod hint;
mod item;
mod list;
mod lock;
mod save;
mod run;
mod new;
//TODO: 添加多语言支持


///根节点
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(long, global=true)]
    ai: bool,
    
    #[arg(long, global=true)]
    all: bool,
    
    #[arg(long, global=true)]
    tool: bool,

    #[arg(long, global=true)]
    pub sort: Option<SortWay>
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///查所有list
    List,
    ///查所有item
    Item(ItemArgs),
    ///增
    #[command(subcommand)]
    New(NewCommand),
    ///删
    #[command(subcommand)]
    Del(DelCommand),
    ///改
    #[command(subcommand)]
    Change(ChangeCommand),
    ///保存本地存档
    Save,
    ///运行
    Run(RunArgs),
    ///消息提醒
    Hint(HintArgs),
    ///锁定指令
    Lock(LockArgs)
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortWay {
    ///时间排序
    #[value(name="t")]
    Time,
    ///优先级排序
    #[value(name="p")]
    Priority
}