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
    ///待删除列表id, 该id是经过映射后的,也就是之前读取过然后保存的Id, 存虚拟id的表中, 值才是真正的id
    #[arg(short, long, num_args = 1.., required = true)]
    pub id: Vec<u64>,
}

#[derive(Args, Debug)]
pub struct DelItemArgs {
    ///待删除项id, 该id是经过映射后的,也就是之前读取过然后保存的Id, 存虚拟id的表中, 值才是真正的id
    #[arg(short, long, num_args = 1.., required = true)]
    pub id: Vec<u64>,
}



