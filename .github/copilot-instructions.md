# GitHub Copilot Instructions for Athira

## Project Overview

Athira is a Git hooks manager and commit message linter written in Rust. It provides a CLI tool for automating development workflows, managing Git hooks, and validating commit messages according to conventional commit standards.

## Project Structure

- **Language**: Rust (2021 edition)
- **CLI Framework**: clap v4 with derive features
- **Configuration**: YAML-based (`hooks.yaml`)
- **Cross-platform**: Supports macOS, Linux, and Windows
- **Package managers**: Available via Cargo, npm, and pip

## Core Components

### 1. Main CLI (`src/main.rs`)

- Uses clap for command parsing with three main command groups:
  - `hooks` - Git hooks management
  - `scripts` - Custom script automation
  - `commit` - Commit message validation
- Command structure: `thira <command-group> <action> [options]`
- Provides helpful error messages and command trees

### 2. Hook Management (`src/hooks.rs`)

- **HookManager struct** manages Git hook installation and configuration
- Supports multiple hooks per Git event (pre-commit, commit-msg, etc.)
- Uses template variables like `${athira}` for dynamic command substitution
- Validates hook names against standard Git hook events
- Supports custom hooks directory (default: `.thira`)

### 3. Configuration (`src/config.rs`)

- **Config struct** with sections:
  - `hooks`: Git hook definitions
  - `scripts`: Custom script configurations
  - `options`: Global settings (auto_install, hooks_dir)
  - `lint`: Commit message validation rules
- **ScriptConfig** supports parallel execution with thread limits
- **CommandConfig** allows per-command environment variables and working directories

### 4. Script Management (`src/scripts.rs`)

- **ScriptManager** handles custom automation scripts
- Supports both sequential and parallel execution
- Real-time output display with terminal UI
- Environment variable and working directory support per command

### 5. Commit Linting (`src/lint.rs`)

- **CommitLinter** validates conventional commit format: `<type>(<scope>): <subject>`
- Configurable allowed types and scopes
- Subject length validation (min/max)
- Body line length validation

## Configuration File (`hooks.yaml`)

```yaml
hooks:
  pre-commit:
    - command: cargo
      args: [test]
    - command: cargo
      args: [clippy]
  commit-msg:
    - command: ${athira}
      args: [commit, validate, $1]

scripts:
  test-all:
    parallel: true
    max_threads: 4
    commands:
      - command: "sh test1.sh"
        description: "Run test script 1"
        working_dir: "."
        env:
          TEST_MODE: "parallel-1"

options:
  auto_install: true
  hooks_dir: .thira

lint:
  types:
    [feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert]
  scopes: [api, ui, db, core, cli, config, deps, tests]
  min_subject_length: 3
  max_subject_length: 72
  max_body_line_length: 100
```

## Key Dependencies

- `clap` - CLI argument parsing
- `serde` + `serde_yaml` - Configuration serialization
- `colored` - Terminal output coloring
- `regex` - Commit message pattern matching
- `crossterm` - Cross-platform terminal manipulation
- `parking_lot` - Efficient synchronization primitives
- `thiserror` + `anyhow` - Error handling

## Coding Patterns and Conventions

### Error Handling

- Uses custom `HookError` enum with `thiserror`
- Result type aliased as `crate::error::Result<T>`
- Specific error variants for different failure modes:
  - `ConfigError` - Configuration validation issues
  - `LintError` - Commit message validation failures
  - `ScriptExecutionError` - Script execution problems
  - `FileError` - File system operations

### CLI Design

- Hierarchical command structure with subcommands
- Colorful output using `colored` crate
- Help text and error suggestions
- Command tree display when no arguments provided

### Configuration Management

- YAML-based configuration with serde
- Default configurations provided
- Auto-installation feature when `auto_install: true`
- Validation methods for configuration integrity

### Template System

- Variable substitution in hook commands (e.g., `${athira}`)
- Support for script references in hooks
- Git parameter passing (e.g., `$1` for commit message file)

## Development Guidelines

### Adding New Features

1. **Commands**: Add to appropriate enum in `main.rs`, implement handler
2. **Configuration**: Extend structs in `config.rs`, add validation
3. **Error Types**: Add specific variants to `HookError` enum
4. **Tests**: Create corresponding test files for new modules

### Code Style

- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Follow Rust naming conventions (snake_case, CamelCase)
- Add comprehensive error handling and user-friendly messages

### Terminal UI

- Use `crossterm` for cross-platform terminal operations
- Implement real-time output for long-running operations
- Provide progress indicators and colored status messages
- Handle terminal size and layout responsively

### Git Integration

- Validate Git repository presence before operations
- Use Git commands via `std::process::Command`
- Handle Git hooks directory configuration (`core.hooksPath`)
- Support both `.git/hooks` and custom directories

## Testing Strategy

- Unit tests for each module
- Integration tests for CLI commands
- Configuration validation tests
- Git hook installation/uninstallation tests
- Commit message linting test cases

## Common Tasks

### Adding a New Hook Type

1. Add to valid hook names in git validation
2. Update default configuration if needed
3. Add documentation example

### Adding New Script Features

1. Extend `CommandConfig` or `ScriptConfig` structs
2. Update YAML serialization/deserialization
3. Implement feature in `ScriptManager`
4. Add CLI command if needed

### Extending Commit Linting

1. Add new rules to `CommitLinter`
2. Update `LinterConfig` with new options
3. Add corresponding error variants
4. Update default configuration

This project emphasizes developer experience, clear error messages, and flexible configuration while maintaining simplicity and reliability.
