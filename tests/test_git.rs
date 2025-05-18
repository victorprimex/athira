mod common;

use athira::git::GitRepo;
use athira::error::HookError;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_git_repo_new() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".thira");
    assert!(repo.is_ok());
    common::cleanup_test_repo(dir);
}

#[test]
fn test_git_repo_new_no_git() {
    let dir = tempfile::tempdir().unwrap();
    let result = GitRepo::new(".thira");
    assert!(matches!(result, Err(HookError::GitNotFound)));
    dir.close().unwrap();
}

#[test]
fn test_set_hooks_path() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".thira").unwrap();
    
    // Set hooks path
    assert!(repo.set_hooks_path().is_ok());
    
    // Verify hooks path was set in git config
    let output = std::process::Command::new("git")
        .args(["config", "--get", "core.hooksPath"])
        .output()
        .unwrap();
    let hooks_path = String::from_utf8_lossy(&output.stdout);
    assert_eq!(hooks_path.trim(), ".thira");
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_install_hook() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".thira").unwrap();
    
    // Install a test hook
    let hook_content = "#!/bin/sh\necho 'test hook'";
    assert!(repo.install_hook("pre-commit", hook_content).is_ok());
    
    // Verify hook was installed
    let hook_path = repo.hooks_dir.join("pre-commit");
    assert!(hook_path.exists());
    assert_eq!(fs::read_to_string(hook_path).unwrap(), hook_content);
    
    // Verify hook is executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::metadata(hook_path).unwrap().permissions().mode();
        assert_eq!(perms & 0o111, 0o111);
    }
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_unset_hooks_path() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".thira").unwrap();
    
    // Set and then unset hooks path
    repo.set_hooks_path().unwrap();
    assert!(repo.unset_hooks_path().is_ok());
    
    // Verify hooks path was unset
    let output = std::process::Command::new("git")
        .args(["config", "--get", "core.hooksPath"])
        .output()
        .unwrap();
    assert!(!output.status.success()); // Config key should not exist
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_validate_hook_name() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".thira").unwrap();
    
    // Test valid hook names
    assert!(repo.validate_hook_name("pre-commit").is_ok());
    assert!(repo.validate_hook_name("commit-msg").is_ok());
    assert!(repo.validate_hook_name("pre-push").is_ok());
    
    // Test invalid hook names
    assert!(repo.validate_hook_name("invalid-hook").is_err());
    assert!(repo.validate_hook_name("").is_err());
    assert!(repo.validate_hook_name("not-a-hook").is_err());
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_get_git_hooks_dir() {
    let dir = common::setup_test_repo();
    
    let hooks_dir = GitRepo::get_git_hooks_dir().unwrap();
    assert_eq!(hooks_dir, PathBuf::from(".git/hooks"));
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_clean_git_hooks() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".thira").unwrap();
    
    // Create test hooks in .git/hooks
    let git_hooks_dir = GitRepo::get_git_hooks_dir().unwrap();
    fs::create_dir_all(&git_hooks_dir).unwrap();
    fs::write(git_hooks_dir.join("pre-commit"), "test content").unwrap();
    fs::write(git_hooks_dir.join("commit-msg"), "test content").unwrap();
    
    // Clean hooks
    let hooks = vec!["pre-commit".to_string(), "commit-msg".to_string()];
    assert!(repo.clean_git_hooks(&hooks).is_ok());
    
    // Verify hooks were removed
    assert!(!git_hooks_dir.join("pre-commit").exists());
    assert!(!git_hooks_dir.join("commit-msg").exists());
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_git_hooks_dir_protection() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".git").unwrap();
    
    // Attempt to unset hooks path with .git directory
    assert!(matches!(repo.unset_hooks_path(), Err(HookError::ConfigError(..))));
    
    common::cleanup_test_repo(dir);
}

#[test]
fn test_uninstall_hook() {
    let dir = common::setup_test_repo();
    let repo = GitRepo::new(".thira").unwrap();
    
    // Install and then uninstall a hook
    repo.install_hook("pre-commit", "test content").unwrap();
    assert!(repo.uninstall_hook("pre-commit").is_ok());
    
    // Verify hook was removed
    assert!(!repo.hooks_dir.join("pre-commit").exists());
    
    // Test uninstalling non-existent hook
    assert!(repo.uninstall_hook("non-existent").is_ok());
    
    common::cleanup_test_repo(dir);
}