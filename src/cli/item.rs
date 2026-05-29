use clap::{Args};

#[derive(Args, Debug)]
pub struct ItemArgs {
    ///查询改id的list下面的所有item, 该id是经过映射后的
    #[arg(long, short, required = true)]
    pub id: u64,
}
 