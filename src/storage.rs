//!数据库操作

use sled::{Db, Tree};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("数据库操作失败: {0}")]
    Sled(#[from] sled::Error),

    #[error("数据不存在: id: {id}")]
    NotFound{ id: u64 },
}


pub struct Storage {
    list_tree: Tree,
    item_tree: Tree,
    pub db: Db
}
impl Storage {
    pub fn new(path: &str) -> Result<Self, StorageError> {
        let db = sled::open(path)?;
        let list_tree = db.open_tree("list")?;
        let item_tree = db.open_tree("item")?;
        Ok(
            Storage { list_tree, item_tree, db }
        )
    }
}