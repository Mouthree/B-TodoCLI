//!数据库操作
#![allow(dead_code)]
use anyhow::{Result, anyhow};

use sled::{Batch, Db, Tree};
use crate::model::{item::ItemData, list::ListData};


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
    pub fn create_id_list(&self) -> Result<u64> {
        Ok(self.db.generate_id()?)
    }

    ///创建项id
    pub fn create_id_item(&self, list_id: u64) -> Result<(u64, u64)> {
        Ok((list_id, self.db.generate_id()?))
    }

    ///保存或创建列表
    pub fn save_or_new_list(&self, list: &ListData) -> Result<()> {
        let id = list.id;
        let list = serde_json::to_vec(list)?;
        self.list_tree.insert(id.to_be_bytes(), list)?;
        Ok(())
    }

    ///保存或创建项
    pub fn save_or_new_item(&self, item: &ItemData) -> Result<()> {
        let id = item.id;
        let id = [id.0.to_be_bytes(), id.1.to_be_bytes()].concat();
        let item = serde_json::to_vec(item)?;
        self.item_tree.insert(id, item)?;
        Ok(())
    }

    ///获取所有列表
    pub fn get_all_list(&self) -> Result<Vec<ListData>> {
        let list = self.list_tree.iter()
            .map(|m| {
                let (_, value) = m?;
                Ok(serde_json::from_slice(&value)?)
            })
            .collect();
        list
    }

    ///获取列表中所有项
    pub fn get_item_from_list(&self, list_id: u64) -> Result<Vec<ItemData>> {
        let list_id = list_id.to_be_bytes();
        let item = self.item_tree.scan_prefix(list_id)
            .map(|m| {
                let (_, value) = m?;
                Ok(serde_json::from_slice(&value)?)
            })
            .collect();
        item
    }

    ///获取单个列表
    pub fn get_a_list(&self, list_id: u64) -> Result<ListData> {
        let list_id = list_id.to_be_bytes();
        let list = self.list_tree.get(list_id)?
            .ok_or_else(|| anyhow!("列表 {} 不存在", u64::from_be_bytes(list_id)))?;
        let list = serde_json::from_slice(&list)?;
        Ok(list)
    }
    
    ///获取单个项
    pub fn get_a_item(&self, item_id: (u64, u64)) -> Result<ItemData> {
        let item_key_bytes = [item_id.0.to_be_bytes(), item_id.1.to_be_bytes()].concat();
        let item = self
            .item_tree
            .get(item_key_bytes)?
            .ok_or_else(|| anyhow!("项{}|{}不存在", item_id.0, item_id.1))?;
        let item = serde_json::from_slice(&item)?;
        Ok(item)
    }

    ///删除一个列表
    pub fn delete_list(&self, list_id: u64) -> Result<()> {
        let list_id = list_id.to_be_bytes();
        let mut batch = Batch::default();
        for m in self.item_tree.scan_prefix(list_id) {
            let (key, _) = m?;          
            batch.remove(key);
        }
        self.item_tree.apply_batch(batch)?;
        self.list_tree.remove(list_id)?;
        Ok(())
    }
    
    ///删除一个项
    pub fn delete_item(&self, item_id: (u64, u64)) -> Result<()> {
        let item_id = [item_id.0.to_be_bytes(), item_id.1.to_be_bytes()].concat();
        self.item_tree.remove(item_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use super::*;

    fn setup_storage() -> Storage {
        let path = tempdir().unwrap();
        Storage::new(path.path().to_str().expect("路径错误")).expect("数据库创建失败")
    }

    #[test]
    fn test_create_id_list() {
        let storage = setup_storage();
        let id1 = storage.create_id_list().unwrap();
        let id2 = storage.create_id_list().unwrap();
        assert!(id2 > id1)
    }

    
    
}