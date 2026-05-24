use clap::{Args, Subcommand};

///公共的属性
#[derive(Args, Debug)]
pub struct CommonArgs {
    pub name: String,
    pub note: String,
    
}

///需要创建的类型
#[derive(Subcommand, Debug)]
pub enum NewCommand {
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
    
}

///打开类型的属性
#[derive(Args, Debug)]
pub struct OpenArgs {
    
}