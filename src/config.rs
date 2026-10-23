use std::{fs, path::Path, str::FromStr};

use anyhow::Context;
use serde::Deserialize;
use serde_tuple::Deserialize_tuple;

#[derive(Debug, PartialEq, Deserialize_tuple)]
pub struct Alias {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub aliases: Vec<Alias>,
}

impl Config {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let json = fs::read_to_string(path)
            .with_context(|| format!("failed to read file: {}", path.display()))?;
        json.parse()
            .with_context(|| format!("failed to parse config file: {}", path.display()))
    }
}

impl FromStr for Config {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).context("failed to parse config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_JSON: &str = r#"
    {
        "aliases": [
            ["gi", "git init"],
            ["cr", "cargo run"]
        ]
    }
    "#;

    #[test]
    fn from_str_parses_all_aliases() {
        let config = Config::from_str(VALID_JSON).unwrap();

        assert_eq!(
            config.aliases,
            vec![
                Alias {
                    name: "gi".into(),
                    command: "git init".into()
                },
                Alias {
                    name: "cr".into(),
                    command: "cargo run".into()
                },
            ]
        );
    }

    #[test]
    fn from_str_rejects_invalid_json() {
        assert!(Config::from_str("{ not json }").is_err());
    }

    #[test]
    fn from_str_rejects_wrong_tuple_arity() {
        let json = r#"{"aliases": [["gi"]]}"#;
        assert!(Config::from_str(json).is_err());
    }
}
