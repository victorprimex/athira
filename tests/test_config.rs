mod common;

use athira::config::{Config, LinterConfig};
use std::collections::HashMap;
use std::fs;

#[test]
fn test_config_load_default() {
    let dir = common::setup_test_repo();
    let config = Config::load().unwrap();
    
    // Verify default hooks
    assert!(config.hooks.contains_key("pre-commit"));
    assert!(config.hooks.contains_key("commit-msg"));
    
    // Verify default scripts
    assert!(config.scripts.contains_key("lint"));
    assert!(config.scripts.contains_key("test"));
    
    // Verify default options
    assert!(config.options.auto_install);
    assert_eq!(config.options.hooks_dir, ".thira");
    
    // Verify default linter config
    assert!(!config.lint.types.is_empty());
    assert!(!config.lint.scopes.is_empty());
    assert_eq!(config.lint.min_subject_length, 3);
    assert_eq!(config.lint.max_subject_length, 72);
    assert_eq!(config.lint.max_body_line_length, 100);
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_config_load_custom() {
    let dir = common::setup_test_repo();
    let config_path = common::create_test_config(&dir);
    
    let config = Config::load().unwrap();
    
    // Verify hooks from test config
    let pre_commit = config.hooks.get("pre-commit").unwrap();
    assert_eq!(pre_commit[0].command, "cargo");
    assert_eq!(pre_commit[0].args, vec!["test"]);
    
    // Verify scripts from test config
    assert_eq!(config.scripts.get("test").unwrap(), "cargo test");
    assert_eq!(config.scripts.get("lint").unwrap(), "cargo clippy");
    
    // Verify linter config from test config
    assert_eq!(config.lint.types, vec!["feat", "fix", "docs"]);
    assert_eq!(config.lint.scopes, vec!["api", "ui", "core"]);
    
    fs::remove_file(config_path).unwrap();
    common::cleanup_test_repo(dir);
}

#[test]
fn test_config_save() {
    let dir = common::setup_test_repo();
    
    let mut config = Config::default();
    
    // Add custom hook
    let mut hooks = HashMap::new();
    hooks.insert(
        "pre-push".to_string(),
        vec![athira::config::Hook {
            command: "cargo".to_string(),
            args: vec!["build".to_string()],
            working_dir: None,
        }],
    );
    config.hooks = hooks;
    
    // Add custom script
    config.scripts.insert("build".to_string(), "cargo build".to_string());
    
    // Modify linter config
    config.lint = LinterConfig {
        types: vec!["test".to_string()],
        scopes: vec!["lib".to_string()],
        min_subject_length: 5,
        max_subject_length: 50,
        max_body_line_length: 80,
    };
    
    // Save and reload
    config.save().unwrap();
    let loaded = Config::load().unwrap();
    
    // Verify saved config
    assert!(loaded.hooks.contains_key("pre-push"));
    assert_eq!(loaded.scripts.get("build").unwrap(), "cargo build");
    assert_eq!(loaded.lint.types, vec!["test"]);
    assert_eq!(loaded.lint.min_subject_length, 5);
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_config_validation() {
    let mut config = Config::default();
    
    // Test invalid hooks dir
    config.options.hooks_dir = ".git".to_string();
    assert!(config.validate().is_err());
    
    // Test empty hook command
    let mut hooks = HashMap::new();
    hooks.insert(
        "pre-commit".to_string(),
        vec![athira::config::Hook {
            command: "".to_string(),
            args: vec![],
            working_dir: None,
        }],
    );
    config.hooks = hooks;
    assert!(config.validate().is_err());
    
    // Test invalid subject length config
    config = Config::default();
    config.lint.max_subject_length = 2; // Less than min_subject_length
    assert!(config.validate().is_err());
}