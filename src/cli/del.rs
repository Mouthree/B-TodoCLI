use clap::{Args, Subcommand};

///删除指令
#[derive(Subcommand, Debug)]
pub enum DelCommand {
    ///列表
    #[command(name = "l")]
    List(DelListArgs),
    ///项
    #[command(name = "i")]
    Item(DelItemArgs),
}


#[derive(Args, Debug)]
pub struct DelListArgs {
    ///待删除列表id, 该id是经过映射后的
    #[arg(short, long, num_args = 1.., required = true)]
    pub id: Vec<u64>,
}

#[derive(Args, Debug)]
pub struct DelItemArgs {
    ///待删除项id, 该id是经过映射后的, 需要前置进行过查询才能调用
    #[arg(short, long, num_args = 1.., required = true)]
    pub id: Vec<u64>,
}



