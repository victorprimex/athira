# Script Management Guide

## Overview

Athira allows you to define and manage reusable scripts that can be integrated into your Git hooks or run independently. This guide explains how to effectively use script management features.

## Managing Scripts

### Adding Scripts

Add a new script using:
```sh
thira scripts add <name> <command>
```

Example:
```sh
thira scripts add lint "cargo clippy --all-features"
thira scripts add test "cargo test --all"
```

### Listing Scripts

View all configured scripts:
```sh
thira scripts list
```

### Running Scripts

Execute a configured script:
```sh
thira scripts run <name>
```

Example:
```sh
thira scripts run lint
```

### Removing Scripts

Delete a configured script:
```sh
thira scripts remove <name>
```

## Script Configuration

Scripts are stored in the `hooks.yaml` file under the `scripts` section:

```yaml
scripts:
  lint: cargo clippy --all-features
  test: cargo test --all
  check: cargo fmt --check
  build: cargo build --release
```

## Using Scripts in Hooks

You can reference scripts in your hook configurations using `${script_name}`:

```yaml
hooks:
  pre-commit:
    - command: ${lint}
    - command: ${test}
```

This will execute the defined scripts during the pre-commit hook.

## Best Practices

### 1. Naming Conventions

- Use descriptive names
- Keep names short but meaningful
- Use kebab-case for multi-word names

Good examples:
```yaml
scripts:
  lint-all: "cargo clippy --workspace"
  test-integration: "cargo test --test integration"
  build-release: "cargo build --release"
```

### 2. Script Organization

- Group related scripts
- Use consistent naming patterns
- Document complex scripts

### 3. Error Handling

- Include error checks
- Provide clear error messages
- Return appropriate exit codes

### 4. Performance

- Keep scripts efficient
- Avoid unnecessary operations
- Use parallel execution when possible

## Advanced Usage

### 1. Combining Scripts

You can combine multiple commands in a single script:

```yaml
scripts:
  check-all: "cargo fmt --check && cargo clippy && cargo test"
```

### 2. Environment Variables

Scripts can use environment variables:

```yaml
scripts:
  build: "RUSTFLAGS='-C target-cpu=native' cargo build --release"
```

### 3. Working Directory

When running scripts through hooks, you can specify a working directory:

```yaml
hooks:
  pre-commit:
    - command: ${test}
      working_dir: tests
```

## Troubleshooting

### Common Issues

1. **Script Not Found**
   - Verify script name: `thira scripts list`
   - Check `hooks.yaml` configuration
   - Ensure correct spelling

2. **Permission Issues**
   - Check file permissions
   - Verify executable paths
   - Use absolute paths when necessary

3. **Environment Problems**
   - Verify environment variables
   - Check PATH configuration
   - Validate dependencies

### Debugging Scripts

1. Run scripts manually first:
   ```sh
   thira scripts run <name>
   ```

2. Check script output:
   ```sh
   RUST_LOG=debug thira scripts run <name>
   ```

3. Verify script configuration:
   ```sh
   cat hooks.yaml
   ```

## Next Steps

- Learn about [Hook Management](hook-management.md)
- Read [Commit Guidelines](commit-guidelines.md)
- Explore advanced configuration options

For more detailed information about any command, use:
```sh
thira scripts --help
```