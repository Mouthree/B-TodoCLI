use derive_builder::Builder;
use serde::{Deserialize, Serialize};



///列表项属性
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ItemData {
    ///(父, 自己)
    pub id: (u64, u64),
    ///条目名字
    pub name: String,
    ///条目备注
    pub note: Option<String>,
    ///条目的具体数据或类型
    pub main_data: ItemVariant,
    ///当前工作状态, 默认为未开始, 未开始的不参与序列化
    pub current_state: CurrentState,
    ///创建时间
    pub create_time: i64,
    ///开始时间
    pub start_time: Option<i64>,
    ///设定的完成时间
    pub dead_line: Option<i64>,
    ///优先级
    pub priority: Option<u8>
}

///项目完成情况
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrentState {
    ///未开始
    NotStart,
    ///工作中
    Working,
    ///已结束
    Over
}

///列表项的变体
//列表项可以有多种
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemVariant {
    ///基本项
    //基础的todolist功能
    Basic,
    ///打开项
    //可以实现自动化操作
    Open {
        ///需要打开的文件路径
        open_file: String,
        ///使用的软件路径
        by_exe: String
    },
}