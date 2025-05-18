mod common;

use athira::config::LinterConfig;
use athira::error::HookError;
use athira::lint::CommitLinter;

#[test]
fn test_valid_commit_message() {
    let config = LinterConfig::default();
    let linter = CommitLinter::new(config);
    
    let valid_messages = [
        "feat(api): add new endpoint",
        "fix(core): resolve memory leak\n\nMore detailed explanation here",
        "docs: update README",
        "chore(deps): update dependencies",
    ];
    
    for message in valid_messages.iter() {
        assert!(linter.validate(message).is_ok());
    }
}

#[test]
fn test_invalid_commit_format() {
    let config = LinterConfig::default();
    let linter = CommitLinter::new(config);
    
    let invalid_messages = [
        "",
        "invalid message",
        "feat:",
        "feat(): empty subject",
        ": no type",
        "(scope): no type",
    ];
    
    for message in invalid_messages.iter() {
        let result = linter.validate(message);
        assert!(matches!(result, Err(HookError::LintError { .. })));
    }
}

#[test]
fn test_invalid_commit_type() {
    let config = LinterConfig {
        types: vec!["feat".into(), "fix".into()],
        ..Default::default()
    };
    let linter = CommitLinter::new(config);
    
    let result = linter.validate("docs(api): update documentation");
    assert!(matches!(result, Err(HookError::LintError { .. })));
}

#[test]
fn test_invalid_commit_scope() {
    let config = LinterConfig {
        scopes: vec!["api".into(), "core".into()],
        ..Default::default()
    };
    let linter = CommitLinter::new(config);
    
    let result = linter.validate("feat(ui): add new component");
    assert!(matches!(result, Err(HookError::LintError { .. })));
}

#[test]
fn test_subject_length_validation() {
    let config = LinterConfig {
        min_subject_length: 10,
        max_subject_length: 20,
        ..Default::default()
    };
    let linter = CommitLinter::new(config);
    
    // Test too short
    let result = linter.validate("feat: abc");
    assert!(matches!(result, Err(HookError::LintError { .. })));
    
    // Test too long
    let result = linter.validate("feat: this is a very long subject that exceeds the maximum length");
    assert!(matches!(result, Err(HookError::LintError { .. })));
    
    // Test just right
    let result = linter.validate("feat: good length");
    assert!(result.is_ok());
}

#[test]
fn test_body_line_length() {
    let config = LinterConfig {
        max_body_line_length: 10,
        ..Default::default()
    };
    let linter = CommitLinter::new(config);
    
    let message = "feat: test\n\nThis line is too long and should cause an error";
    let result = linter.validate(message);
    assert!(matches!(result, Err(HookError::LintError { .. })));
    
    let message = "feat: test\n\nShort\nline";
    let result = linter.validate(message);
    assert!(result.is_ok());
}

#[test]
fn test_empty_lines_in_body() {
    let config = LinterConfig::default();
    let linter = CommitLinter::new(config);
    
    let message = "feat: test\n\nFirst paragraph\n\nSecond paragraph";
    let result = linter.validate(message);
    assert!(result.is_ok());
}

#[test]
fn test_custom_config() {
    let config = LinterConfig {
        types: vec!["custom".into()],
        scopes: vec!["test".into()],
        min_subject_length: 5,
        max_subject_length: 10,
        max_body_line_length: 20,
    };
    let linter = CommitLinter::new(config);
    
    // Valid according to custom config
    assert!(linter.validate("custom(test): ok").is_ok());
    
    // Invalid type according to custom config
    assert!(linter.validate("feat(test): not ok").is_err());
    
    // Invalid scope according to custom config
    assert!(linter.validate("custom(wrong): not ok").is_err());
    
    // Invalid subject length according to custom config
    assert!(linter.validate("custom(test): too long subject").is_err());
}