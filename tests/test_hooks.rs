mod common;

use athira::error::HookError;
use athira::hooks::HookManager;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_hook_manager_new() {
    let dir = common::setup_test_repo();
    let result = HookManager::new();
    assert!(result.is_ok());
    common::cleanup_test_repo(dir);
}

#[test]
fn test_hook_manager_new_no_git() {
    let dir = tempfile::tempdir().unwrap();
    let result = HookManager::new();
    assert!(matches!(result, Err(HookError::GitNotFound)));
    dir.close().unwrap();
}

#[test]
fn test_install_hooks() {
    let dir = common::setup_test_repo();
    common::create_test_config(&dir);
    
    let manager = HookManager::new().unwrap();
    assert!(manager.install_hooks().is_ok());
    
    // Verify hooks were installed
    let hooks_dir = PathBuf::from(".thira");
    assert!(hooks_dir.exists());
    assert!(hooks_dir.join("pre-commit").exists());
    assert!(hooks_dir.join("commit-msg").exists());
    
    // Verify hooks are executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let pre_commit_perms = fs::metadata(hooks_dir.join("pre-commit"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(pre_commit_perms & 0o111, 0o111);
    }
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_uninstall_hooks() {
    let dir = common::setup_test_repo();
    common::create_test_config(&dir);
    
    let mut manager = HookManager::new().unwrap();
    manager.install_hooks().unwrap();
    
    // Verify hooks are installed
    assert!(PathBuf::from(".thira").exists());
    
    // Uninstall hooks
    assert!(manager.uninstall_hooks().is_ok());
    
    // Verify hooks directory is removed
    assert!(!PathBuf::from(".thira").exists());
    
    // Verify config file is removed
    assert!(!PathBuf::from("hooks.yaml").exists());
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_add_hook() {
    let dir = common::setup_test_repo();
    let mut manager = HookManager::new().unwrap();
    
    // Add a new hook
    assert!(manager
        .add_hook(
            "pre-push".to_string(),
            "cargo".to_string(),
            vec!["build".to_string()]
        )
        .is_ok());
    
    // Verify hook was added
    let hooks = manager.get_hooks();
    assert!(hooks.contains_key("pre-push"));
    let hook = &hooks["pre-push"][0];
    assert_eq!(hook.command, "cargo");
    assert_eq!(hook.args, vec!["build"]);
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_add_invalid_hook() {
    let dir = common::setup_test_repo();
    let mut manager = HookManager::new().unwrap();
    
    // Try to add an invalid hook
    let result = manager.add_hook(
        "invalid-hook".to_string(),
        "command".to_string(),
        vec![],
    );
    assert!(matches!(result, Err(HookError::InvalidHook(..))));
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_run_script() {
    let dir = common::setup_test_repo();
    common::create_test_config(&dir);
    
    let manager = HookManager::new().unwrap();
    
    // Test running a configured script
    assert!(manager.run_script("test").is_ok());
    
    // Test running a non-existent script
    assert!(matches!(
        manager.run_script("nonexistent"),
        Err(HookError::ScriptExecutionError { .. })
    ));
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_hooks_path() {
    let dir = common::setup_test_repo();
    let manager = HookManager::new().unwrap();
    
    // Test default hooks path
    let path = manager.get_hooks_path().unwrap();
    assert_eq!(path, ".thira");
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_unset_hooks_path() {
    let dir = common::setup_test_repo();
    let mut manager = HookManager::new().unwrap();
    
    // Change hooks path
    manager.unset_hooks_path().unwrap();
    
    // Verify path was changed to .git/hooks
    let path = manager.get_hooks_path().unwrap();
    assert_eq!(path, ".git/hooks");
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_git_hook_cleanup() {
    let dir = common::setup_test_repo();
    let mut manager = HookManager::new().unwrap();
    
    // Install hooks in custom directory
    common::create_test_config(&dir);
    manager.install_hooks().unwrap();
    
    // Create a test hook in .git/hooks
    let git_hooks_dir = PathBuf::from(".git/hooks");
    fs::create_dir_all(&git_hooks_dir).unwrap();
    fs::write(git_hooks_dir.join("pre-commit"), "test content").unwrap();
    
    // Uninstall hooks should clean up both directories
    manager.uninstall_hooks().unwrap();
    
    assert!(!git_hooks_dir.join("pre-commit").exists());
    assert!(!PathBuf::from(".thira").exists());
    
    common::cleanup_test_repo(dir);
}