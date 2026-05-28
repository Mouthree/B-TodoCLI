#![allow(non_snake_case)]

use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};
use tracing::info;

use crate::ai::{ChatRequest, DeepseekRequest};

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
    loop {
        let readline = rl.readline(left_sign);
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
                    "exit" => {
                        break;
                    },
                    "exit ai" => {
                        left_sign = "[  ]>> ";
                        ai_flag = false;
                        continue;
                    },
                    _ => {
                        info!("不是基础指令")
                    }
                }
                //如果是ai模式那么需要将输入给到ai然后返回的结果作为参数传入cli中
                if ai_flag {
                    let mut cr = ChatRequest::new();
                    let api_key = "sk-a35a1fc4524c41be99498c4264a49416";
                    let deep = DeepseekRequest::new(api_key.to_owned());
                    cr.insert_user_input("现在进行一下测试, 随便输出一些什么");
                    deep.send_to_ds(&mut cr).await?;
                    return Ok(());
                }
                //执行一次
                if let Err(e) = cli::run(&line) {
                    eprintln!("错误: {e}");   // 打印错误但不退出
                }
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