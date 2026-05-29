#![allow(non_snake_case)]

use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};
use tracing::info;

use crate::{ai::{ChatRequest, DeepseekRequest}, config::CONFIG};

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
    let mut cr = ChatRequest::new();
    let api_key = &CONFIG.api_key;
    let deep = DeepseekRequest::new(api_key.to_owned());
    let mut output = String::new();
    
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
                        info!("不是基础指令")
                    }
                }
                //如果是ai模式那么需要将输入给到ai然后返回的结果作为参数传入cli中
                if ai_flag {
                    info!("传给ai进行处理");
                    cr.insert_user_input(line.as_str());
                    info!("向ds发送");
                    output = deep.send_to_ds(&mut cr).await?;
                    continue;
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