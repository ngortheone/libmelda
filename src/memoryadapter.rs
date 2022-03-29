// Melda - Delta State JSON CRDT
// Copyright (C) 2021-2022 Amos Brocco <amos.brocco@supsi.ch>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use crate::adapter::Adapter;
use anyhow::Result;
use std::{cell::RefCell, collections::BTreeMap, sync::Mutex};

pub struct MemoryAdapter {
    data: Mutex<RefCell<BTreeMap<String, Vec<u8>>>>,
}

impl MemoryAdapter {
    pub fn new() -> Self {
        MemoryAdapter {
            data: Mutex::new(RefCell::new(BTreeMap::<String, Vec<u8>>::new())),
        }
    }
}

impl Adapter for MemoryAdapter {
    fn read_object(&self, key: &str, offset: usize, length: usize) -> Result<Vec<u8>> {
        let mem = self.data.lock().unwrap();
        let d = mem.borrow();
        let data = d.get(key).unwrap();
        if offset == 0 && length == 0 {
            Ok(data.clone())
        } else {
            Ok(data.as_slice()[offset..offset + length].to_vec())
        }
    }

    fn write_object(&self, key: &str, data: &[u8]) -> Result<()> {
        let mem = self.data.lock().unwrap();
        let mut d = mem.borrow_mut();
        if !d.contains_key(key) {
            d.insert(key.to_string(), data.to_vec());
        }
        Ok(())
    }
    fn list_objects(&self, ext: &str) -> Result<Vec<String>> {
        let list: Vec<String> = self
            .data
            .lock()
            .unwrap()
            .borrow()
            .keys()
            .filter(|x| return x.ends_with(ext))
            .map(|x| x.strip_suffix(ext).unwrap().to_string())
            .collect();
        Ok(list)
    }
}
