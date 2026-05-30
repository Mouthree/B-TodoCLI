use anyhow::Result;
use clap::{Command, Parser, Subcommand};
use clap::{ValueEnum};
use tracing::info;
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
mod save;
mod run;
mod new;

///根节点
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    ///显示全部信息
    #[arg(long, global=true)]
    all: bool,

    ///排序方式
    #[arg(long, global=true)]
    sort: Option<SortWay>
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///查所有list
    List,
    ///查指定list下所有item
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
    ///测试指令
    Test
}

///排序方式
#[derive(Debug, Clone, ValueEnum)]
pub enum SortWay {
    ///时间排序
    #[value(name="t")]
    Time,
    ///优先级排序
    #[value(name="p")]
    Priority
}

pub fn run(args: &str) -> Result<()> {
    //切割输入
    let mut args = shellwords::split(args)?;
    //第一个给空, 默认第一个是给程序名留着的
    args.insert(0, "".to_string());
    let cli = match Cli::try_parse_from(args) {
        Ok(c) => {
            info!("输入解码成功");
            c
        },
        Err(e) => {
            eprint!("{e}");
            info!("输入格式错误");
            return Ok(());
        }
    };
    //TODO: 如果输出是结果的话交给ai让ai整理后输出
    match &cli.command {
        Commands::Test => {
            print_all_help::<Cli>();
        },
        Commands::List => {
            //TODO: 添加用户友好输出, 就是输出表格的形式, 找找有什么库可以实现
            //TODO: 查完之后要将映射后的表存起来, 方便直接输入012这样的序号进行后续的处理
        },
        Commands::Item(item_args) => todo!(),
        Commands::New(new_command) => todo!(),
        Commands::Del(del_command) => todo!(),
        Commands::Change(change_command) => todo!(),
        Commands::Save => todo!(),
        Commands::Run(run_args) => todo!(),
        Commands::Hint(hint_args) => todo!(),
    }
    Ok(())
}


fn print_recursive(cmd: &mut Command) {
    // 打印当前命令的完整帮助（-h 只给摘要，--help 给完整版，这里用 print_long_help）
    cmd.print_long_help().unwrap();
    println!(); // 命令之间空行分隔，方便 AI 阅读

    // 收集子命令名称，避免借用冲突
    let names: Vec<String> = cmd
        .get_subcommands()
        .map(|sc| sc.get_name().to_owned())
        .collect();

    for name in names {
        if let Some(mut sub) = cmd.find_subcommand_mut(&name) {
            print_recursive(&mut sub);
        }
    }
}

/// 你可以直接调用的唯一接口：自动获取命令树并打印全部帮助
pub fn print_all_help<C: Parser>() {
    let mut cmd = C::command();
    print_recursive(&mut cmd);
}

///判断是否是有效指令
pub fn is_valid_command(args: &str) -> Result<bool> {
    let mut args = shellwords::split(args)?;
    args.insert(0, "".to_string());
    Ok(Cli::try_parse_from(args).is_ok())
}