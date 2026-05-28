#![allow(non_snake_case)]

use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};

pub mod model;
pub mod cli;
pub mod storage;
pub mod config;
pub mod ai;

pub fn start(start_items: Option<String>) -> Result<()> {
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
                        println!("未定义行为")
                    }
                }
                //如果是ai模式那么需要将输入给到ai然后返回的结果作为参数传入cli中
                if ai_flag {
                    
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