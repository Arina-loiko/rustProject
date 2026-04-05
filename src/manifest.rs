use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::error::ResolverError;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub package: Package,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

impl Manifest {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ResolverError> {
        let text = fs::read_to_string(path)?;
        let m: Manifest = toml::from_str(&text)?;
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_manifest() {
        let src = r#"
[package]
name = "demo"
version = "0.1.0"

[dependencies]
serde = "1.0.0"
"#;
        let m: Manifest = toml::from_str(src).unwrap();
        assert_eq!(m.package.name, "demo");
        assert_eq!(m.package.version, "0.1.0");
        assert_eq!(m.dependencies.get("serde").map(String::as_str), Some("1.0.0"));
    }

    #[test]
    fn rejects_broken_toml() {
        let src = "= = not toml";
        assert!(toml::from_str::<Manifest>(src).is_err());
    }
}
