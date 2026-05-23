//!数据库操作
#![allow(dead_code)]
use anyhow::Result;
use sled::{Db, Tree};
use crate::model::{item::ItemData, list::TodoListData};


pub struct Storage {
    list_tree: Tree,
    item_tree: Tree,
    pub db: Db,
}
impl Storage {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        let list_tree = db.open_tree("list")?;
        let item_tree = db.open_tree("item")?;
        Ok(
            Storage {
                list_tree,
                item_tree,
                db,
            }
        )
    }

    ///创建列表id
    fn create_id_list(&self) -> Result<u64> {
        Ok(self.db.generate_id()?)
    }

    ///创建项id
    fn create_id_item(&self, list_id: u64) -> Result<(u64, u64)> {
        Ok((list_id, self.db.generate_id()?))
    }

    ///保存或创建列表
    fn save_or_new_list(&self, list: &TodoListData) -> Result<()> {
        let id = list.id;
        let list = serde_json::to_vec(list)?;
        self.list_tree.insert(id.to_be_bytes(), list)?;
        Ok(())
    }

    ///保存或创建项
    fn save_or_new_item(&self, item: &ItemData) -> Result<()> {
        let id = item.id;
        let id = [id.0.to_be_bytes(), id.1.to_be_bytes()].concat();
        let item = serde_json::to_vec(item)?;
        self.list_tree.insert(id, item)?;
        Ok(())
    }
    
}
