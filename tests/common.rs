use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

pub fn setup_test_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    
    // Initialize git repo
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(&dir)
        .output()
        .unwrap();

    // Configure git user for commits
    std::process::Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&dir)
        .output()
        .unwrap();

    std::process::Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .output()
        .unwrap();

    dir
}

pub fn create_test_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, content).unwrap();
    path
}

pub fn stage_file(dir: &TempDir, path: &PathBuf) {
    std::process::Command::new("git")
        .args(&["add", path.to_str().unwrap()])
        .current_dir(&dir)
        .output()
        .unwrap();
}

pub fn commit_file(dir: &TempDir, message: &str) -> std::process::Output {
    std::process::Command::new("git")
        .args(&["commit", "-m", message])
        .current_dir(&dir)
        .output()
        .unwrap()
}

pub fn cleanup_test_repo(dir: TempDir) {
    dir.close().unwrap();
}

pub fn create_test_config(dir: &TempDir) -> PathBuf {
    let config = r#"
hooks:
  pre-commit:
    - command: cargo
      args: ["test"]
    - command: cargo
      args: ["clippy"]
  commit-msg:
    - command: thira
      args: ["commit", "validate", "$1"]

scripts:
  test: cargo test
  lint: cargo clippy

options:
  auto_install: true
  hooks_dir: .thira

lint:
  types:
    - feat
    - fix
    - docs
  scopes:
    - api
    - ui
    - core
  min_subject_length: 3
  max_subject_length: 72
  max_body_line_length: 100
"#;

    let config_path = dir.path().join("hooks.yaml");
    fs::write(&config_path, config).unwrap();
    config_path
}