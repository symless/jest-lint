use glob_match::glob_match;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

const CONFIG_FILENAME: &str = ".jest_lint.json";

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub ignored_modules: Vec<String>,
}

impl Config {
    pub fn is_ignored(&self, module: &str) -> bool {
        self.ignored_modules.iter().any(|pattern| {
            pattern == module
                || glob_match(pattern, module)
                || glob_match(pattern, module.trim_start_matches("./"))
                || module
                    .rsplit_once('/')
                    .is_some_and(|(_, name)| glob_match(pattern, name))
        })
    }
}

pub fn find_config(start: &Path) -> Config {
    let mut dir = if start.is_file() {
        start.parent().map(Path::to_path_buf)
    } else {
        Some(start.to_path_buf())
    };

    while let Some(current) = dir {
        let config_path = current.join(CONFIG_FILENAME);
        if config_path.exists() {
            return load_config(&config_path);
        }
        dir = current.parent().map(PathBuf::from);
    }

    Config::default()
}

fn load_config(path: &Path) -> Config {
    let contents = fs::read_to_string(path).expect("Error reading config file.");
    serde_json::from_str(&contents).expect("Error parsing config file.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with(patterns: &[&str]) -> Config {
        Config {
            ignored_modules: patterns.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_exact_match() {
        let config = config_with(&["zod"]);
        assert!(config.is_ignored("zod"));
        assert!(!config.is_ignored("zod-utils"));
    }

    #[test]
    fn test_single_star_matches_one_level() {
        let config = config_with(&["@mui/*"]);
        assert!(config.is_ignored("@mui/material"));
        // Single * should NOT match nested paths
    }

    #[test]
    fn test_double_star_matches_nested() {
        let config = config_with(&["@mui/**"]);
        assert!(config.is_ignored("@mui/material"));
        assert!(config.is_ignored("@mui/material/CircularProgress"));
    }

    #[test]
    fn test_scss_module_pattern() {
        let config = config_with(&["*.module.scss"]);
        assert!(config.is_ignored("./layout.module.scss"));
        assert!(config.is_ignored("../../app/purchase/page.module.scss"));
        assert!(!config.is_ignored("./layout.scss"));
    }

    #[test]
    fn test_types_pattern() {
        let config = config_with(&["**/types/*"]);
        assert!(config.is_ignored("../../types/route"));
        assert!(config.is_ignored("../../../types/product"));
    }

    #[test]
    fn test_next_subpaths() {
        let config = config_with(&["next/**"]);
        assert!(config.is_ignored("next/server"));
        assert!(config.is_ignored("next/font/local"));
    }
}
