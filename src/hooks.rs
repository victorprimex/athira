use crate::config::{Config, ScriptConfig};
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

    // pub fn run_script(&self, name: &str) -> Result<()> {
    //     // Use map_err for custom error construction
    //     let script =
    //         self.config
    //             .scripts
    //             .get(name)
    //             .ok_or_else(|| HookError::ScriptExecutionError {
    //                 script_name: name.to_string(),
    //                 reason: "Script not found".to_string(),
    //             })?;

    //     // Use map_err to convert the error with context
    //     let status = std::process::Command::new("sh")
    //         .arg("-c")
    //         .arg(script)
    //         .status()
    //         .map_err(|e| HookError::ScriptExecutionError {
    //             script_name: name.to_string(),
    //             reason: e.to_string(),
    //         })?;

    //     if !status.success() {
    //         return Err(HookError::ScriptExecutionError {
    //             script_name: name.to_string(),
    //             reason: format!("Script failed with status {}", status),
    //         });
    //     }

    //     Ok(())
    // }
    //



    // pub fn run_script(&self, name: &str) -> Result<()> {
    //     use crossterm::{
    //         cursor,
    //         execute,
    //         terminal::{self, Clear, ClearType},
    //         style::{Color, Print, SetForegroundColor},
    //     };
    //     use parking_lot::Mutex;
    //     use std::io::{stdout};
    //     use std::sync::Arc;
    //     use std::thread;
    //     use std::time::Instant;

    //     // Create a mutex-wrapped stdout for thread-safe access
    //     let stdout = Arc::new(Mutex::new(stdout()));

    //     // Initialize terminal
    //     {
    //         let mut stdout = stdout.lock();
    //         execute!(stdout, terminal::EnterAlternateScreen)?;
    //     }
    //     terminal::enable_raw_mode()?;

    //     // Start timing
    //     let start_time = Instant::now();

    //     // Get the script
    //     let script = self.config
    //         .scripts
    //         .get(name)
    //         .ok_or_else(|| HookError::ScriptExecutionError {
    //             script_name: name.to_string(),
    //             reason: "Script not found".to_string(),
    //         })?;

    //     // Split commands
    //     let commands: Vec<&str> = script
    //         .split(|c| c == ';' || c == '&')
    //         .map(str::trim)
    //         .filter(|s| !s.is_empty())
    //         .collect();

    //     if commands.is_empty() {
    //         return Ok(());
    //     }

    //     let script = Arc::new(commands);
    //     let mut handles = vec![];

    //     // Calculate screen layout
    //     let (term_width, term_height) = terminal::size()?;
    //     let section_height = term_height / script.len() as u16;

    //     // Clear screen and show initial layout
    //     {
    //         let mut stdout = stdout.lock();
    //         execute!(stdout, Clear(ClearType::All))?;

    //         for (idx, cmd) in script.iter().enumerate() {
    //             let y_pos = idx as u16 * section_height;
    //             execute!(
    //                 stdout,
    //                 cursor::MoveTo(0, y_pos),
    //                 SetForegroundColor(Color::Cyan),
    //                 Print("┌".to_string() + &"─".repeat(term_width as usize - 2) + "┐"),
    //                 cursor::MoveTo(2, y_pos),
    //                 Print(format!(" Command {}: {} ", idx + 1, cmd)),
    //             )?;
    //         }
    //     }

    //     // Spawn threads for each command
    //     for (idx, cmd) in script.iter().enumerate() {
    //         let cmd = cmd.to_string();
    //         let name = name.to_string();
    //         let stdout = Arc::clone(&stdout);
    //         let y_pos = idx as u16 * section_height;
    //         let term_width = term_width;

    //         let handle = thread::spawn(move || -> Result<()> {
    //             let command_start = Instant::now();
    //             let mut child = std::process::Command::new("sh")
    //                 .arg("-c")
    //                 .arg(&cmd)
    //                 .stdout(std::process::Stdio::piped())
    //                 .stderr(std::process::Stdio::piped())
    //                 .spawn()
    //                 .map_err(|e| HookError::ScriptExecutionError {
    //                     script_name: name.clone(),
    //                     reason: e.to_string(),
    //                 })?;

    //             let mut current_line = y_pos + 1;

    //             // Process output in real-time
    //             if let Some(child_stdout) = child.stdout.take() {
    //                 let reader = std::io::BufReader::new(child_stdout);
    //                 for line in std::io::BufRead::lines(reader) {
    //                     if let Ok(line) = line {
    //                         let mut stdout = stdout.lock();
    //                         execute!(
    //                             stdout,
    //                             cursor::MoveTo(1, current_line),
    //                             Clear(ClearType::CurrentLine),
    //                             Print(&line)
    //                         )?;
    //                         current_line = (current_line + 1).min(y_pos + section_height - 1);
    //                     }
    //                 }
    //             }

    //             let status = child.wait().map_err(|e| HookError::ScriptExecutionError {
    //                 script_name: name.clone(),
    //                 reason: e.to_string(),
    //             })?;

    //             if !status.success() {
    //                 return Err(HookError::ScriptExecutionError {
    //                     script_name: name,
    //                     reason: format!("Command '{}' failed with status {}", cmd, status),
    //                 });
    //             }

    //             let duration = command_start.elapsed();
    //             let mut stdout = stdout.lock();
    //             execute!(
    //                 stdout,
    //                 cursor::MoveTo(term_width - 20, y_pos),
    //                 SetForegroundColor(Color::Green),
    //                 Print(format!("[{:.2?}]", duration))
    //             )?;

    //             Ok(())
    //         });

    //         handles.push(handle);
    //     }

    //     // Wait for all threads to complete
    //     let mut all_successful = true;
    //     for handle in handles {
    //         match handle.join().map_err(|_| HookError::ScriptExecutionError {
    //             script_name: name.to_string(),
    //             reason: "Thread panicked".to_string(),
    //         })? {
    //             Ok(_) => (),
    //             Err(e) => {
    //                 all_successful = false;
    //                 let mut stdout = stdout.lock();
    //                 execute!(
    //                     stdout,
    //                     cursor::MoveTo(0, term_height - 2),
    //                     SetForegroundColor(Color::Red),
    //                     Print(format!("Error: {}", e))
    //                 )?;
    //             }
    //         }
    //     }

    //     // Show final status
    //     let total_duration = start_time.elapsed();
    //     {
    //         let mut stdout = stdout.lock();
    //         execute!(
    //             stdout,
    //             cursor::MoveTo(0, term_height - 1),
    //             SetForegroundColor(if all_successful { Color::Green } else { Color::Red }),
    //             Print(format!(
    //                 "Execution {} in {:.2?}",
    //                 if all_successful { "completed" } else { "failed" },
    //                 total_duration
    //             ))
    //         )?;

    //         // Wait for user input before closing
    //         execute!(stdout, cursor::MoveTo(0, term_height))?;
    //     }

    //     terminal::disable_raw_mode()?;
    //     {
    //         let mut stdout = stdout.lock();
    //         execute!(stdout, terminal::LeaveAlternateScreen)?;
    //     }

    //     if !all_successful {
    //         return Err(HookError::ScriptExecutionError {
    //             script_name: name.to_string(),
    //             reason: "One or more commands failed".to_string(),
    //         });
    //     }

    //     Ok(())
    // }

    pub fn run_script(&self, name: &str) -> Result<()> {
        use crossterm::{
            cursor,
            execute,
            terminal::{self, Clear, ClearType},
            style::{Color, Print, SetForegroundColor},
        };
        use parking_lot::Mutex;
        use std::io::stdout;
        use std::sync::Arc;
        use std::thread::{self, JoinHandle};  // Add JoinHandle import
        use std::time::Instant;
        use std::sync::atomic::{AtomicUsize, Ordering};

        // Create a mutex-wrapped stdout for thread-safe access
        let stdout = Arc::new(Mutex::new(stdout()));

        // Initialize terminal
        {
            let mut stdout = stdout.lock();
            execute!(stdout, terminal::EnterAlternateScreen)?;
        }
        terminal::enable_raw_mode()?;

        // Start timing
        let start_time = Instant::now();

        // Get the script configuration
        let script_config = self.config
            .scripts
            .get(name)
            .ok_or_else(|| HookError::ScriptExecutionError {
                script_name: name.to_string(),
                reason: "Script not found".to_string(),
            })?;

        if script_config.commands.is_empty() {
            return Ok(());
        }

        let commands = Arc::new(script_config.commands.clone());
        let mut handles: Vec<JoinHandle<Result<()>>> = vec![];  // Add type annotation here

        // Calculate screen layout
        let (term_width, term_height) = terminal::size()?;
        let section_height = term_height / commands.len() as u16;

        // Clear screen and show initial layout
        {
            let mut stdout = stdout.lock();
            execute!(stdout, Clear(ClearType::All))?;

            for (idx, cmd) in commands.iter().enumerate() {
                let y_pos = idx as u16 * section_height;
                execute!(
                    stdout,
                    cursor::MoveTo(0, y_pos),
                    SetForegroundColor(Color::Cyan),
                    Print("┌".to_string() + &"─".repeat(term_width as usize - 2) + "┐"),
                    cursor::MoveTo(2, y_pos),
                    Print(format!(
                        " Command {}: {} {}",
                        idx + 1,
                        cmd.command,
                        cmd.description.as_ref().map(|d| format!("({})", d)).unwrap_or_default()
                    )),
                )?;
            }
        }

        // For parallel execution with max threads control
        let active_threads = Arc::new(AtomicUsize::new(0));

        // Execute commands
        // Execute commands
        for (idx, cmd) in commands.iter().enumerate() {
            let cmd = cmd.clone();
            let name = name.to_string();
            let stdout = Arc::clone(&stdout);
            let y_pos = idx as u16 * section_height;
            let term_width = term_width;
            let active_threads = Arc::clone(&active_threads);

            // If parallel execution is disabled, wait for previous command to complete
            if !script_config.parallel {
                // Take ownership and join the last handle if it exists
                if let Some(last_handle) = handles.pop() {
                    match last_handle.join().map_err(|_| HookError::ScriptExecutionError {
                        script_name: name.clone(),
                        reason: "Thread panicked".to_string(),
                    })? {
                        Ok(_) => (),
                        Err(e) => {
                            // If sequential execution fails, return immediately
                            let mut stdout = stdout.lock();
                            execute!(
                                stdout,
                                cursor::MoveTo(0, term_height - 2),
                                SetForegroundColor(Color::Red),
                                Print(format!("Error: {}", e))
                            )?;
                            return Err(HookError::ScriptExecutionError {
                                script_name: name.clone(),
                                reason: "Previous command failed".to_string(),
                            });
                        }
                    }
                }
            } else {
                // Wait if we've reached max threads
                while script_config.parallel && active_threads.load(Ordering::SeqCst) >= script_config.max_threads {
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            }

            active_threads.fetch_add(1, Ordering::SeqCst);

            let handle = thread::spawn(move || -> Result<()> {
                let command_start = Instant::now();

                // Build command with working directory and environment variables
                let mut child = std::process::Command::new("sh");
                child.arg("-c").arg(&cmd.command);

                // Set working directory if specified
                if let Some(dir) = &cmd.working_dir {
                    child.current_dir(dir);
                }

                // Set environment variables
                for (key, value) in &cmd.env {
                    child.env(key, value);
                }

                // Configure stdio
                let mut child = child
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
                    .map_err(|e| HookError::ScriptExecutionError {
                        script_name: name.clone(),
                        reason: e.to_string(),
                    })?;

                let mut current_line = y_pos + 1;

                // Process output in real-time
                if let Some(child_stdout) = child.stdout.take() {
                    let reader = std::io::BufReader::new(child_stdout);
                    for line in std::io::BufRead::lines(reader) {
                        if let Ok(line) = line {
                            let mut stdout = stdout.lock();
                            execute!(
                                stdout,
                                cursor::MoveTo(1, current_line),
                                Clear(ClearType::CurrentLine),
                                Print(&line)
                            )?;
                            current_line = (current_line + 1).min(y_pos + section_height - 1);
                        }
                    }
                }

                let status = child.wait().map_err(|e| HookError::ScriptExecutionError {
                    script_name: name.clone(),
                    reason: e.to_string(),
                })?;

                if !status.success() {
                    return Err(HookError::ScriptExecutionError {
                        script_name: name,
                        reason: format!("Command '{}' failed with status {}", cmd.command, status),
                    });
                }

                let duration = command_start.elapsed();
                let mut stdout = stdout.lock();
                execute!(
                    stdout,
                    cursor::MoveTo(term_width - 20, y_pos),
                    SetForegroundColor(Color::Green),
                    Print(format!("[{:.2?}]", duration))
                )?;

                active_threads.fetch_sub(1, Ordering::SeqCst);
                Ok(())
            });


            handles.push(handle);
        }

        // Wait for all remaining threads to complete
        let mut all_successful = true;
        while let Some(handle) = handles.pop() {
            match handle.join().map_err(|_| HookError::ScriptExecutionError {
                script_name: name.to_string(),
                reason: "Thread panicked".to_string(),
            })? {
                Ok(_) => (),
                Err(e) => {
                    all_successful = false;
                    let mut stdout = stdout.lock();
                    execute!(
                        stdout,
                        cursor::MoveTo(0, term_height - 2),
                        SetForegroundColor(Color::Red),
                        Print(format!("Error: {}", e))
                    )?;
                    if !script_config.parallel {
                        break; // Exit early for sequential execution
                    }
                }
            }
        }

        // Show final status
        let total_duration = start_time.elapsed();
        {
            let mut stdout = stdout.lock();
            execute!(
                stdout,
                cursor::MoveTo(0, term_height - 1),
                SetForegroundColor(if all_successful { Color::Green } else { Color::Red }),
                Print(format!(
                    "Execution {} in {:.2?} ({})",
                    if all_successful { "completed" } else { "failed" },
                    total_duration,
                    if script_config.parallel {
                        format!("parallel, max {} threads", script_config.max_threads)
                    } else {
                        "sequential".to_string()
                    }
                ))
            )?;

            // Wait for user input before closing
            execute!(stdout, cursor::MoveTo(0, term_height))?;
        }



        terminal::disable_raw_mode()?;
        {
            let mut stdout = stdout.lock();
            execute!(stdout, terminal::LeaveAlternateScreen)?;
        }

        if !all_successful {
            return Err(HookError::ScriptExecutionError {
                script_name: name.to_string(),
                reason: "One or more commands failed".to_string(),
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

    pub fn get_scripts(&self) -> &HashMap<String, ScriptConfig> {
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
        for (name, script_config) in &self.config.scripts {
            let var = format!("${{{}}}", name);
            if result.contains(&var) {
                // Use the first command's command string
                if let Some(first_cmd) = script_config.commands.first() {
                    result = result.replace(&var, &first_cmd.command);
                }
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
            .or_default()
            .push(hook);


        // Just save - config.save() will handle auto_install
        self.config.save()?;

        Ok(())
    }


}
