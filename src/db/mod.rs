use crate::{bincoding::TOMBSTONE, lsmtree::LSMTree, wal::WriteAheadLog};
use std::collections::BTreeMap;
use std::fs;

pub struct TKDB {
    pub sstable_dir: String,
    pub threshold: usize,
    pub wal_path: String,
}

impl TKDB {
    pub fn new() -> Self {
        Self {
            sstable_dir: "sstables".to_string(),
            threshold: 3,
            wal_path: "sstables/wal.log".to_string(),
        }
    }

    pub fn with_sstable_dir(mut self, dir: &str) -> Self {
        self.sstable_dir = dir.to_string();
        // ensure WAL path follows the sstable directory
        self.wal_path = format!("{}/wal.log", self.sstable_dir);
        self
    }

    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn with_wal_path(mut self, path: &str) -> Self {
        self.wal_path = path.to_string();
        self
    }

    pub fn build(self) -> std::io::Result<LSMTree> {
        fs::create_dir_all(&self.sstable_dir)?;

        let mut memtable = BTreeMap::new();
        if let Ok(ops) = WriteAheadLog::replay(&self.wal_path) {
            for (key, value_opt) in ops {
                match value_opt {
                    Some(val) => memtable.insert(key, val),
                    None => memtable.insert(key, TOMBSTONE.to_string()),
                };
            }
        }

        Ok(LSMTree {
            memtable,
            sstable_counter: 0,
            wal: WriteAheadLog::new(&self.wal_path)?,
        })
    }
}
