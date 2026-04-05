use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ResolverError {
    Io(io::Error),
    TomlParse(toml::de::Error),
    JsonParse(serde_json::Error),
    PackageNotFound { name: String, version: String },
    VersionConflict { name: String, wanted: String, existing: String },
    Cycle(Vec<String>),
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolverError::Io(e) => write!(f, "ошибка ввода-вывода: {}", e),
            ResolverError::TomlParse(e) => write!(f, "ошибка разбора TOML: {}", e),
            ResolverError::JsonParse(e) => write!(f, "ошибка разбора JSON: {}", e),
            ResolverError::PackageNotFound { name, version } => {
                write!(f, "пакет не найден в реестре: {} {}", name, version)
            }
            ResolverError::VersionConflict { name, wanted, existing } => {
                write!(f, "конфликт версий для {}: требуется {}, уже выбрана {}", name, wanted, existing)
            }
            ResolverError::Cycle(path) => {
                write!(f, "обнаружен цикл: {}", path.join(" -> "))
            }
        }
    }
}

impl std::error::Error for ResolverError {}

impl From<io::Error> for ResolverError {
    fn from(e: io::Error) -> Self {
        ResolverError::Io(e)
    }
}

impl From<toml::de::Error> for ResolverError {
    fn from(e: toml::de::Error) -> Self {
        ResolverError::TomlParse(e)
    }
}

impl From<serde_json::Error> for ResolverError {
    fn from(e: serde_json::Error) -> Self {
        ResolverError::JsonParse(e)
    }
}
