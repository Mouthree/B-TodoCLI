use clap::{ArgGroup, Args};

#[derive(Args, Debug)]
#[command(group = ArgGroup::new("hint").required(true).multiple(true))]
pub struct HintArgs {
    ///添加需要提醒的item,可以是多个
    #[arg(long, short, group = "hint")]
    pub item: Option<Vec<u64>>,

    ///提前提醒时间, 单位是分钟
    #[arg(long, short, default_value_t = 5)]
    pub ahead: u64,
    
    ///提示信息, 可选项, 如果没有添加待提醒的item, 那么必须添加
    #[arg(long, short, group = "hint")]
    pub message: Option<String>,
    
    ///提醒时间
    #[arg(long, short)]
    pub time: Option<String>
}