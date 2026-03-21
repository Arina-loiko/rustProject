use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

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
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let r: Registry = serde_json::from_str(&text)?;
        Ok(r)
    }

    pub fn get(&self, name: &str, version: &str) -> Option<&PackageInfo> {
        self.entries.get(name)?.get(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_registry() {
        let src = r#"{
            "serde": {
                "1.0.0": { "dependencies": { "serde_derive": "1.0.0" } }
            },
            "serde_derive": {
                "1.0.0": { "dependencies": {} }
            }
        }"#;
        let r: Registry = serde_json::from_str(src).unwrap();
        let info = r.get("serde", "1.0.0").unwrap();
        assert_eq!(info.dependencies.get("serde_derive").map(String::as_str), Some("1.0.0"));
        assert!(r.get("serde_derive", "1.0.0").is_some());
        assert!(r.get("tokio", "1.0.0").is_none());
    }

    #[test]
    fn rejects_broken_json() {
        let src = "{ this is not json";
        assert!(serde_json::from_str::<Registry>(src).is_err());
    }
}
