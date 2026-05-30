#![allow(non_snake_case)]

use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};
use tracing::info;

use crate::{ai::{ChatRequest, DeepseekRequest}, cli::is_valid_command, config::CONFIG};

pub mod model;
pub mod cli;
pub mod storage;
pub mod config;
pub mod ai;

pub async fn start(start_items: Option<String>) -> Result<()> {
    
    //有参数就执行一次, 没有就是循环执行
    if let Some(input) = start_items {
        return cli::run(&input);
    }
    //获取输入行
    let mut rl = DefaultEditor::new()?;
    let mut left_sign = "[  ]>> ";
    let mut ai_flag = false;
    
    //ai相关数据
    //系统提示词
    let t = "
        根据下面的提示, 来将我输入的需求转化为指令返回, 只需要返回指令就行
        涉及到时间的部分都使用时间戳
    
        根节点
        
        Usage: B-TodoCLI [OPTIONS] <COMMAND>
        
        Commands:
        list    查所有list
        item    查指定list下所有item
        new     增
        del     删
        change  改
        save    保存本地存档至github
        run     运行
        hint    消息提醒
        test    测试指令
        help    Print this message or the help of the given subcommand(s)
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        查所有list
        
        Usage: list [OPTIONS]
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        查指定list下所有item
        
        Usage: item [OPTIONS] --id <ID>
        
        Options:
        -i, --id <ID>
                查询改id的list下面的所有item, 该id是经过映射后的
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        增
        
        Usage: new [OPTIONS] <COMMAND>
        
        Commands:
        l     列表
        i     项
        help  Print this message or the help of the given subcommand(s)
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        列表
        
        Usage: l [OPTIONS] --name <NAME>
        
        Options:
        -n, --name <NAME>
                名字
        
        -t, --note <NOTE>
                注释
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        项
        
        Usage: i [OPTIONS] <COMMAND>
        
        Commands:
        b     基础项
        o     打开文件的项
        help  Print this message or the help of the given subcommand(s)
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        基础项
        
        Usage: b [OPTIONS] --name <NAME> --list <LIST>
        
        Options:
        -n, --name <NAME>
                名字
        
        -t, --note <NOTE>
                注释
        
        -l, --list <LIST>
                添加到的列表的id
        
        -d, --ddl <DEAD_LINE>
                截止时间
        
        -s, --start <START_TIME>
                开始时间
        
            --pri <PRIORITY>
                优先级
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        打开文件的项
        
        Usage: o [OPTIONS] --name <NAME> --list <LIST> --path <PATH>... --exe <EXE>
        
        Options:
        -n, --name <NAME>
                名字
        
        -t, --note <NOTE>
                注释
        
        -l, --list <LIST>
                添加到的列表的id
        
        -d, --ddl <DEAD_LINE>
                截止时间
        
        -s, --start <START_TIME>
                开始时间
        
            --pri <PRIORITY>
                优先级
        
            --path <PATH>...
                待打开的路径
        
        -e, --exe <EXE>
                用该软件打开
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        Print this message or the help of the given subcommand(s)
        
        Usage: help [COMMAND]...
        
        Arguments:
        [COMMAND]...
                Print help for the subcommand(s)
        
        Print this message or the help of the given subcommand(s)
        
        Usage: help [COMMAND]...
        
        Arguments:
        [COMMAND]...
                Print help for the subcommand(s)
        
        删
        
        Usage: del [OPTIONS] <COMMAND>
        
        Commands:
        l     列表
        i     项
        help  Print this message or the help of the given subcommand(s)
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        列表
        
        Usage: l [OPTIONS] --id <ID>...
        
        Options:
        -i, --id <ID>...
                待删除列表id, 该id是经过映射后的
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        项
        
        Usage: i [OPTIONS] --id <ID>...
        
        Options:
        -i, --id <ID>...
                待删除项id, 该id是经过映射后的, 需要前置进行过查询才能调用
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        Print this message or the help of the given subcommand(s)
        
        Usage: help [COMMAND]...
        
        Arguments:
        [COMMAND]...
                Print help for the subcommand(s)
        
        改
        
        Usage: change [OPTIONS] <COMMAND>
        
        Commands:
        l     列表
        i     项
        help  Print this message or the help of the given subcommand(s)
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        列表
        
        Usage: l [OPTIONS] --id <ID>
        
        Options:
        -i, --id <ID>
                待修改的 id
        
        -n, --name <NAME>
                名字
        
        -t, --note <NOTE>
                注释
        
            --pri <PRIORITY>
                优先级
        
        -l, --last-open <LAST_OPEN>
                最近打开时间
        
        -c, --current-state <CURRENT_STATE>
                完成情况
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        项
        
        Usage: i [OPTIONS] <COMMAND>
        
        Commands:
        b     基本项
        o     打开项
        help  Print this message or the help of the given subcommand(s)
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        基本项
        
        Usage: b [OPTIONS] --id <ID>
        
        Options:
        -i, --id <ID>
                待修改的 id
        
        -n, --name <NAME>
                名字
        
        -t, --note <NOTE>
                注释
        
            --pri <PRIORITY>
                优先级
        
        -d, --ddl <DEAD_LINE>
                截止时间
        
        -s, --start <START_TIME>
                开始时间
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        打开项
        
        Usage: o [OPTIONS] --id <ID>
        
        Options:
        -i, --id <ID>
                待修改的 id
        
        -n, --name <NAME>
                名字
        
        -t, --note <NOTE>
                注释
        
            --pri <PRIORITY>
                优先级
        
        -d, --ddl <DEAD_LINE>
                截止时间
        
        -s, --start <START_TIME>
                开始时间
        
        -o, --open-file <OPEN_FILE>
                待打开路径
        
        -b, --by-exe <BY_EXE>
                所使用软件
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        Print this message or the help of the given subcommand(s)
        
        Usage: help [COMMAND]...
        
        Arguments:
        [COMMAND]...
                Print help for the subcommand(s)
        
        Print this message or the help of the given subcommand(s)
        
        Usage: help [COMMAND]...
        
        Arguments:
        [COMMAND]...
                Print help for the subcommand(s)
        
        保存本地存档至github
        
        Usage: save [OPTIONS]
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        运行
        
        Usage: run [OPTIONS] --id <ID>... --run-type <RUN_TYPE>
        
        Options:
        -i, --id <ID>...
                待运行id
        
        -r, --run-type <RUN_TYPE>
                运行列表还是项
        
                Possible values:
                - list: 运行整个列表中所有可以运行的
                - item: 运行单独的一个项
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        消息提醒
        
        Usage: hint [OPTIONS] <--item <ITEM>|--message <MESSAGE>>
        
        Options:
        -i, --item <ITEM>
                添加需要提醒的item,可以是多个
        
        -a, --ahead <AHEAD>
                提前提醒时间, 单位是分钟
        
                [default: 5]
        
        -m, --message <MESSAGE>
                提示信息, 可选项, 如果没有添加待提醒的item, 那么必须添加
        
        -t, --time <TIME>
                提醒时间
        
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        测试指令
        
        Usage: test [OPTIONS]
        
        Options:
            --all
                显示全部信息
        
            --sort <SORT>
                排序方式
        
                Possible values:
                - t: 时间排序
                - p: 优先级排序
        
        -h, --help
                Print help (see a summary with '-h')
        
        Print this message or the help of the given subcommand(s)
        
        Usage: help [COMMAND]...
        
        Arguments:
        [COMMAND]...
                Print help for the subcommand(s)";
    let mut cr = ChatRequest::new(t);
    let api_key = &CONFIG.api_key;
    let deep = DeepseekRequest::new(api_key.to_owned());
    let mut output = String::from("");
    
    loop {
        let readline = rl.readline_with_initial(left_sign, (&output, ""));
        match readline {
            Ok(line) => {
                //基础指令
                //TODO: 多指令同时运行, xxxx | xxxxx | xxxxx 这样
                match line.as_str() {
                    "ai" => {
                        left_sign = "[AI]>> ";
                        ai_flag = true;
                        continue;
                    },
                    "exit" | "e" => {
                        break;
                    },
                    "exit ai" | "e ai" => {
                        left_sign = "[  ]>> ";
                        ai_flag = false;
                        continue;
                    },
                    _ => {
                        
                    }
                }

                //判断是否为指令
                if is_valid_command(line.as_str())? {
                    //执行一次
                    info!("是指令");
                    if let Err(e) = cli::run(&line) {
                        eprintln!("错误: {e}");   // 打印错误但不退出
                    }
                } else {
                    info!("需要传给ai");
                    //如果是ai模式那么需要将输入给到ai然后返回的结果作为参数传入cli中
                    if ai_flag {
                        info!("传给ai进行处理");
                        cr.insert_user_input(line.as_str());
                        info!("向ds发送");
                        output = deep.send_to_ds(&mut cr).await?;
                        continue;
                    }
                }
                output = String::from("");
            }
            //正常退出
            Err(ReadlineError::Eof) => {
                break;
            }
            _ => {
                
            }
        }
    }
    Ok(())
}