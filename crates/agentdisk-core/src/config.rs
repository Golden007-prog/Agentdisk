//! `config.toml` in the config dir. Everything is optional.
//!
//! ```toml
//! projects = ["C:/Users/me/Projects", "D:/games"]
//! worktree_days = 7
//! build_days = 14
//! max_depth = 6
//! min_free_gb = 20
//! max_worktrees = 20
//! adapters = ["C:/Users/me/agentdisk-adapters"]
//! ```

use crate::adapters::{parse_toml, TVal};
use crate::paths;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub projects: Vec<PathBuf>,
    pub worktree_days: u64,
    pub build_days: u64,
    pub max_depth: usize,
    pub min_free_gb: u64,
    pub max_worktrees: usize,
    pub adapters: Vec<PathBuf>,
    pub warnings: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            projects: Vec::new(),
            worktree_days: 7,
            build_days: 14,
            max_depth: 6,
            min_free_gb: 20,
            max_worktrees: 20,
            adapters: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

pub fn path() -> PathBuf {
    paths::config_dir().join("config.toml")
}

pub fn load() -> Config {
    let Ok(text) = std::fs::read_to_string(path()) else {
        return Config::default();
    };
    parse_config(&text)
}

fn parse_config(text: &str) -> Config {
    let mut c = Config::default();
    match parse_toml(text) {
        Err(e) => c.warnings.push(format!("config.toml: {e}")),
        Ok(doc) => {
            for (k, v) in &doc.top {
                match k.as_str() {
                    "projects" | "adapters" => {
                        let folders: Option<Vec<PathBuf>> = match v {
                            TVal::Str(folder) => Some(vec![PathBuf::from(folder)]),
                            TVal::Arr(values) => {
                                let folders: Vec<PathBuf> = values
                                    .iter()
                                    .filter_map(|v| v.as_str().map(PathBuf::from))
                                    .collect();
                                let ignored = values.len() - folders.len();
                                if ignored > 0 {
                                    let entries = if ignored == 1 {
                                        "entry that is"
                                    } else {
                                        "entries that are"
                                    };
                                    c.warnings.push(format!("config.toml: {k} must be a list of folders, like [\"path/to/folder\"]; ignored {ignored} {entries} not text"));
                                }
                                Some(folders)
                            }
                            _ => None,
                        };
                        match folders {
                            Some(folders) if k == "projects" => c.projects = folders,
                            Some(folders) => c.adapters = folders,
                            None => c.warnings.push(format!("config.toml: {k} must be a list of folders, like [\"path/to/folder\"]")),
                        }
                    }
                    "worktree_days" => {
                        c.worktree_days = number(v, k, &mut c.warnings).unwrap_or(c.worktree_days)
                    }
                    "build_days" => {
                        c.build_days = number(v, k, &mut c.warnings).unwrap_or(c.build_days)
                    }
                    "min_free_gb" => {
                        c.min_free_gb = number(v, k, &mut c.warnings).unwrap_or(c.min_free_gb)
                    }
                    "max_depth" | "max_worktrees" => {
                        if let Some(n) = number(v, k, &mut c.warnings) {
                            match usize::try_from(n) {
                                Ok(value) if k == "max_depth" => c.max_depth = value,
                                Ok(value) => c.max_worktrees = value,
                                Err(_) => c.warnings.push(format!(
                                    "config.toml: {k} is too large for this platform"
                                )),
                            }
                        }
                    }
                    other => c
                        .warnings
                        .push(format!("config.toml: unknown key '{other}'")),
                }
            }
            for (section, fields) in &doc.tables {
                for (key, _) in fields {
                    c.warnings.push(format!("config.toml: settings must come before any [section]; '{key}' under [{section}] is ignored"));
                }
            }
        }
    }
    c
}

fn number(value: &TVal, key: &str, warnings: &mut Vec<String>) -> Option<u64> {
    let number = value.as_int().and_then(|n| u64::try_from(n).ok());
    if number.is_none() {
        let unit = match key {
            "worktree_days" | "build_days" => " of days",
            "min_free_gb" => " of GiB",
            _ => "",
        };
        warnings.push(format!(
            "config.toml: {key} must be a whole number{unit} (0 or more)"
        ));
    }
    number
}

pub const EXAMPLE: &str = r#"# agentdisk config
# Folders that hold your repositories (scanned for worktrees and build folders).
projects = []
# Days without changes before something may be planned for cleanup.
worktree_days = 7
build_days = 14
# How deep to look inside project folders.
max_depth = 6
# Warning limits for the fast check (free space in GiB, total agent worktrees).
min_free_gb = 20
max_worktrees = 20
# Extra folders with adapter .toml files.
adapters = []
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config_keeps_all_values_without_warnings() {
        let cfg = parse_config(
            r#"
projects = ["workspace/one", "workspace/two"]
adapters = ["adapters"]
worktree_days = 0
build_days = 3
max_depth = 9
min_free_gb = 0
max_worktrees = 2
"#,
        );
        assert!(cfg.warnings.is_empty(), "{:?}", cfg.warnings);
        assert_eq!(
            cfg.projects,
            vec![
                PathBuf::from("workspace/one"),
                PathBuf::from("workspace/two")
            ]
        );
        assert_eq!(cfg.adapters, vec![PathBuf::from("adapters")]);
        assert_eq!(
            (cfg.worktree_days, cfg.build_days, cfg.max_depth),
            (0, 3, 9)
        );
        assert_eq!((cfg.min_free_gb, cfg.max_worktrees), (0, 2));
        let defaults = parse_config(EXAMPLE);
        assert!(defaults.warnings.is_empty());
        assert!(defaults.projects.is_empty() && defaults.adapters.is_empty());
        assert_eq!(
            (
                defaults.worktree_days,
                defaults.build_days,
                defaults.max_depth
            ),
            (7, 14, 6)
        );
        assert_eq!((defaults.min_free_gb, defaults.max_worktrees), (20, 20));
    }

    #[test]
    fn invalid_config_numbers_warn_and_keep_defaults() {
        for (key, expected) in [
            ("worktree_days", 7),
            ("build_days", 14),
            ("max_depth", 6),
            ("min_free_gb", 20),
            ("max_worktrees", 20),
        ] {
            for value in ["-3", "\"soon\"", "true", "[]"] {
                let cfg = parse_config(&format!("{key} = {value}"));
                assert_eq!(cfg.warnings.len(), 1, "{key} = {value}: {:?}", cfg.warnings);
                assert!(
                    cfg.warnings[0].starts_with(&format!("config.toml: {key} must be ")),
                    "{:?}",
                    cfg.warnings
                );
                let actual = match key {
                    "worktree_days" => cfg.worktree_days,
                    "build_days" => cfg.build_days,
                    "min_free_gb" => cfg.min_free_gb,
                    "max_worktrees" => cfg.max_worktrees as u64,
                    _ => cfg.max_depth as u64,
                };
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn invalid_config_folder_lists_warn_and_keep_defaults() {
        for key in ["projects", "adapters"] {
            for value in ["1", "true"] {
                let cfg = parse_config(&format!("{key} = {value}"));
                assert_eq!(cfg.warnings.len(), 1, "{key} = {value}: {:?}", cfg.warnings);
                assert!(
                    cfg.warnings[0]
                        .starts_with(&format!("config.toml: {key} must be a list of folders")),
                    "{:?}",
                    cfg.warnings
                );
                assert!(cfg.projects.is_empty() && cfg.adapters.is_empty());
            }
        }
    }

    #[test]
    fn config_folder_strings_remain_supported() {
        for key in ["projects", "adapters"] {
            let cfg = parse_config(&format!("{key} = \"workspace\""));
            assert!(cfg.warnings.is_empty(), "{:?}", cfg.warnings);
            let folders = if key == "projects" {
                cfg.projects
            } else {
                cfg.adapters
            };
            assert_eq!(folders, vec![PathBuf::from("workspace")]);
        }
    }

    #[test]
    fn config_mixed_folder_lists_keep_text_entries_and_warn() {
        for key in ["projects", "adapters"] {
            for (value, paths, ignored) in [
                (r#"["first", 5, "second"]"#, vec!["first", "second"], 1),
                (
                    r#"["first", true, [], "second"]"#,
                    vec!["first", "second"],
                    2,
                ),
                ("[1, true]", vec![], 2),
            ] {
                let cfg = parse_config(&format!("{key} = {value}"));
                let folders = if key == "projects" {
                    cfg.projects
                } else {
                    cfg.adapters
                };
                assert_eq!(
                    folders,
                    paths.into_iter().map(PathBuf::from).collect::<Vec<_>>()
                );
                assert_eq!(cfg.warnings.len(), 1);
                assert!(cfg.warnings[0].contains(key), "{:?}", cfg.warnings);
                assert!(
                    cfg.warnings[0].contains(&format!("ignored {ignored} ")),
                    "{:?}",
                    cfg.warnings
                );
            }
        }
    }

    #[test]
    fn config_table_keys_warn_without_overriding_top_level_values() {
        let cfg = parse_config("worktree_days = 2\n[defaults]\nworktree_days = 1\nunknown = true\n[[extra]]\nbuild_days = 0\n");
        assert_eq!(cfg.worktree_days, 2);
        assert_eq!(cfg.build_days, 14);
        assert_eq!(cfg.warnings, vec![
            "config.toml: settings must come before any [section]; 'worktree_days' under [defaults] is ignored",
            "config.toml: settings must come before any [section]; 'unknown' under [defaults] is ignored",
            "config.toml: settings must come before any [section]; 'build_days' under [extra] is ignored",
        ]);
    }

    #[test]
    fn existing_config_parse_and_unknown_key_warnings_remain() {
        let cfg = parse_config("unknown = true");
        assert_eq!(cfg.warnings, vec!["config.toml: unknown key 'unknown'"]);
        let cfg = parse_config("max_depth = 2.5");
        assert_eq!(cfg.warnings.len(), 1);
        assert!(cfg.warnings[0].starts_with("config.toml: line 1: "));
        assert_eq!(cfg.max_depth, 6);
    }
}
