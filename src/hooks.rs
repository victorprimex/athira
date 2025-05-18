use crate::config::Config;
use crate::error::HookError;
use crate::error::Result;
use crate::git::GitRepo;
use crate::lint::CommitLinter;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct HookManager {
    config: Config,
    repo: GitRepo,
}

impl HookManager {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let repo = GitRepo::new(&config.options.hooks_dir)?;

        Ok(Self { config, repo })
    }

    pub fn run_script(&self, name: &str) -> Result<()> {
        // Use map_err for custom error construction
        let script =
            self.config
                .scripts
                .get(name)
                .ok_or_else(|| HookError::ScriptExecutionError {
                    script_name: name.to_string(),
                    reason: "Script not found".to_string(),
                })?;

        // Use map_err to convert the error with context
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .status()
            .map_err(|e| HookError::ScriptExecutionError {
                script_name: name.to_string(),
                reason: e.to_string(),
            })?;

        if !status.success() {
            return Err(HookError::ScriptExecutionError {
                script_name: name.to_string(),
                reason: format!("Script failed with status {}", status),
            });
        }

        Ok(())
    }

    pub fn validate_commit_message(&self, message_file: &str) -> Result<()> {
        let message = std::fs::read_to_string(message_file).map_err(|e| HookError::FileError {
            path: PathBuf::from(message_file),
            source: e,
        })?;

        let linter = CommitLinter::new(self.config.lint.clone());
        linter.validate(&message)
    }

    pub fn add_script(&mut self, name: String, command: String) -> Result<()> {
        self.config.add_script(name, command)
    }

    pub fn remove_script(&mut self, name: &str) -> Result<()> {
        self.config.remove_script(name)
    }

    pub fn get_hooks(&self) -> &HashMap<String, Vec<crate::config::Hook>> {
        &self.config.hooks
    }

    pub fn get_scripts(&self) -> &HashMap<String, String> {
        &self.config.scripts
    }

    pub fn install_hooks(&self) -> Result<()> {
        self.config.validate()?;

        // Get list of existing hook files
        let existing_hooks = std::fs::read_dir(&self.repo.hooks_dir)
            .map_err(|e| HookError::FileError {
                path: self.repo.hooks_dir.clone(),
                source: e,
            })?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name())
            .filter_map(|name| name.into_string().ok())
            .collect::<Vec<_>>();

        // Remove hooks that are no longer in config
        for hook_name in existing_hooks {
            if !self.config.hooks.contains_key(&hook_name) {
                self.repo.uninstall_hook(&hook_name)?;
            }
        }


        // Clean up old hooks in .git/hooks before installing new ones
        self.repo
            .clean_git_hooks(&self.config.hooks.keys().cloned().collect::<Vec<_>>())?;

        // Install new hooks
        for (name, hooks) in &self.config.hooks {
            self.repo.validate_hook_name(name)?;
            let script = self.generate_hook_script(hooks);
            self.repo.install_hook(name, &script)?;
        }

        // Configure Git to use our hooks directory
        if !self.config.options.hooks_dir.contains(".git/hooks") {
            self.repo.set_hooks_path()?;
        }

        Ok(())
    }

    fn generate_hook_script(&self, hooks: &[crate::config::Hook]) -> String {
        let mut script = String::from("#!/bin/sh\n\n");

        for hook in hooks {
            let mut command = self.substitute_variables(&hook.command);

            // Special handling for the commit-msg hook
            if command.contains("thira") && hook.args.contains(&"commit".to_string()) {
                // Use the binary path for commit message validation
                script.push_str(&format!(
                    "{} commit validate \"$1\"\n",
                    std::env::current_exe().unwrap().display()
                ));
                script.push_str("if [ $? -ne 0 ]; then\n");
                script.push_str("  exit 1\n");
                script.push_str("fi\n\n");
                continue;
            }

            if !hook.args.is_empty() {
                let args = hook
                    .args
                    .iter()
                    .map(|arg| self.substitute_variables(arg))
                    .collect::<Vec<_>>()
                    .join(" ");
                command.push_str(&format!(" {}", args));
            }

            script.push_str(&format!("{}\n", command));
            script.push_str("if [ $? -ne 0 ]; then\n");
            script.push_str("  exit 1\n");
            script.push_str("fi\n\n");
        }

        script
    }

    fn substitute_variables(&self, input: &str) -> String {
        let mut result = input.to_string();

        // Replace ${script_name} with actual script command
        for (name, command) in &self.config.scripts {
            let var = format!("${{{}}}", name);
            if result.contains(&var) {
                result = result.replace(&var, command);
            }
        }

        result
    }

    pub fn get_hooks_path(&self) -> Result<String> {
        // First check Git config
        let output = std::process::Command::new("git")
            .args(["config", "--get", "core.hooksPath"])
            .output()?;

        // If Git config exists and is valid, use it
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path_buf = std::path::PathBuf::from(&path);

            if path_buf.exists() {
                return Ok(path);
            }
        }

        // Otherwise return the path from our config
        Ok(self.config.options.hooks_dir.clone())
    }

    pub fn uninstall_hooks(&mut self) -> Result<()> {
        // First uninstall all hooks from current location
        for name in self.config.hooks.keys() {
            self.repo.uninstall_hook(name)?;
        }

        // Clean up any hooks in .git/hooks
        self.repo
            .clean_git_hooks(&self.config.hooks.keys().cloned().collect::<Vec<_>>())?;

        // Unset Git's core.hooksPath
        self.repo.unset_hooks_path()?;

        // Remove the config file
        if std::path::Path::new("hooks.yaml").exists() {
            std::fs::remove_file("hooks.yaml")?;
        }

        Ok(())
    }

    pub fn unset_hooks_path(&mut self) -> Result<()> {
        // Clean up hooks from old location
        self.repo
            .clean_git_hooks(&self.config.hooks.keys().cloned().collect::<Vec<_>>())?;

        // Unset Git's core.hooksPath
        self.repo.unset_hooks_path()?;

        // Update config to use default .git/hooks directory
        self.config.options.hooks_dir = ".git/hooks".to_string();

        // Save config without auto-installing
        let auto_install = self.config.options.auto_install;
        self.config.options.auto_install = false;
        self.config.save()?;
        self.config.options.auto_install = auto_install;

        // Update repo reference
        self.repo = GitRepo::new(&self.config.options.hooks_dir)?;

        Ok(())
    }

    pub fn add_hook(&mut self, name: String, command: String, args: Vec<String>) -> Result<()> {
        let hook = crate::config::Hook {
            command,
            args,
            working_dir: None,
        };

        self.repo.validate_hook_name(&name)?;

        self.config
            .hooks
            .entry(name.clone())
            .or_insert_with(Vec::new)
            .push(hook);

        // Just save - config.save() will handle auto_install
        self.config.save()?;

        Ok(())
    }


}
