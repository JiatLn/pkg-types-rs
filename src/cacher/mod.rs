use crate::PackageJson;
use std::{collections::HashMap, path::PathBuf};

pub struct Cacher<T> {
    value: HashMap<PathBuf, T>,
}

impl Cacher<PackageJson> {
    pub fn get(&mut self, key: &PathBuf) -> PackageJson {
        let value = self.value.get(key);
        match value {
            Some(v) => v.clone(),
            None => {
                let pkg: PackageJson = PackageJson::from_path(key).unwrap();
                self.set(key.clone(), &pkg);
                pkg
            }
        }
    }
    fn set(&mut self, key: PathBuf, value: &PackageJson) {
        self.value.insert(key, value.clone());
    }

    pub fn new() -> Self {
        Cacher {
            value: HashMap::new(),
        }
    }
}
