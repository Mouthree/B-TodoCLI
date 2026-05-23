use derive_builder::Builder;
use serde::{Deserialize, Serialize};


///列表属性
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ListData {
    ///列表id
    pub id: u64,
    ///列表名字
    pub name: String,
    ///创建时间
    pub create_time: i64,
    ///最近打开时间
    pub last_open: i64,
    ///完成情况
    pub current_state: Option<f64>,
    ///列表注释
    pub note: Option<String>,
    ///优先级
    pub priority: Option<u8> 
}