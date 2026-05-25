use clap::{Parser, Subcommand};
use clap::{ValueEnum};
use crate::cli::cd::CdArgs;
use crate::cli::change::ChangeCommand;
use crate::cli::del::DelCommand;
use crate::cli::hint::HintArgs;
use crate::cli::item::ItemArgs;
use crate::cli::new::NewCommand;
use crate::cli::run::RunArgs;
mod change;
mod del;
mod hint;
mod item;
mod list;
mod cd;
mod save;
mod run;
mod new;


///根节点
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    ///开启ai模式, 此情况下直接输入自然语言, 调用deepseek API来转为实际指令
    #[arg(long, global=true)]
    ai: bool,

    ///显示全部信息
    #[arg(long, global=true)]
    all: bool,

    ///调用模式, 由别的软件调用来实现相关操作, 不输入则为循环模式
    #[arg(long, global=true)]
    tool: bool,

    ///排序方式
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
    ///保存本地存档至github
    Save,
    ///运行
    Run(RunArgs),
    ///消息提醒
    Hint(HintArgs),
    ///处理默认指令, ai处理的情况下该指令禁用
    Cd(CdArgs),
    ///进入TUI模式
    Tui
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

//TODO: 循环指令的话用一个栈来实现