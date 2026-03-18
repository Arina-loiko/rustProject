use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct PackageInfo {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(transparent)]
pub struct Registry {
    pub entries: HashMap<String, HashMap<String, PackageInfo>>,
}

impl Registry {
    pub fn get(&self, name: &str, version: &str) -> Option<&PackageInfo> {
        self.entries.get(name)?.get(version)
    }
}
