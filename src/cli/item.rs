use clap::{Args};

#[derive(Args, Debug)]
pub struct ItemArgs {
    ///查询改id的list下面的所有item, 该id是经过映射后的,也就是之前读取过然后保存的Id, 存虚拟id的表中, 值才是真正的id
    #[arg(long, short, required = true)]
    pub id: u64,
}
 