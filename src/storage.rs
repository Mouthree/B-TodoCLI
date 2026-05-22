//!数据库操作

use sled::{Db, Tree};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("数据库操作失败: {0}")]
    Sled(#[from] sled::Error),
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
    
    ///创建列表id
    fn create_id_list(&self) -> Result<u64, StorageError> {
        Ok(self.db.generate_id()?)
    }
    
    ///创建项id
    fn create_id_item(&self, list_id: u64) -> Result<(u64, u64), StorageError> {
        Ok((list_id, self.db.generate_id()?))
    }

    
}