use clap::{Args, Subcommand};

///删除指令
#[derive(Subcommand, Debug)]
pub enum DelCommand {
    #[command(name = "l")]
    List(DelListArgs),
    #[command(name = "i")]
    Item(DelItemArgs),
}


#[derive(Args, Debug)]
pub struct DelListArgs {
    ///待删除列表id
    #[arg(short, long, num_args = 1.., required = true)]
    pub id: Vec<u64>,
}

#[derive(Args, Debug)]
pub struct DelItemArgs {
    ///待删除项id
    #[arg(short, long, num_args = 1.., required = true)]
    pub id: Vec<u64>,
}



