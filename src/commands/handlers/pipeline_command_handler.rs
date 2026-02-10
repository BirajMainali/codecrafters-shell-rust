use crate::commands::{command_handler::CommandHandler, supported_command::SupportedCommand};
use anyhow::{bail, Ok, Result};
use std::sync::Arc;

pub struct PipelineCommandHandler {
    handlers: Vec<Arc<dyn CommandHandler>>,
}

impl PipelineCommandHandler {
    pub fn new(handlers: Vec<Arc<dyn CommandHandler>>) -> Self {
        Self { handlers }
    }
}

impl CommandHandler for PipelineCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::Pipeline { .. })
    }

    /// Executes a pipeline of commands, supporting both built-ins (echo, type) and
    /// external binaries (ls, cat).
    fn handle(&self, cmd: &SupportedCommand) -> Result<String> {
        let SupportedCommand::Pipeline { commands } = cmd else {
            bail!("Unsupported command passed to PipelineCommandHandler");
        };

        let mut prev_stdout = Some(std::process::Stdio::inherit());
        let mut prev_output_buffer: Option<String> = None;
        let mut child_processes = Vec::new();

        let num_commands = commands.len();

        let unspecified_handler = self.handlers.iter()
            .find(|h| h.can_handle(&SupportedCommand::Unspecified { cmd: String::new(), args: vec![] }))
            .and_then(|h| h.as_any().downcast_ref::<crate::commands::unspecified_command_handler::UnspecifiedCommandHandler>())
            .expect("UnspecifiedCommandHandler must be available");

        for (i, command) in commands.iter().enumerate() {
            let is_last = i == num_commands - 1;

            match command {
                SupportedCommand::Unspecified { cmd, args } => {
                    let (stdin, buffer_to_write) = if let Some(buf) = prev_output_buffer.take() {
                        (std::process::Stdio::piped(), Some(buf))
                    } else {
                        (
                            prev_stdout.take().unwrap_or(std::process::Stdio::inherit()),
                            None,
                        )
                    };

                    let stdout = if is_last {
                        std::process::Stdio::inherit()
                    } else {
                        std::process::Stdio::piped()
                    };

                    let mut child = unspecified_handler.spawn_process(
                        cmd,
                        args,
                        stdin,
                        stdout,
                        std::process::Stdio::inherit(),
                    )?;

                    if let Some(content) = buffer_to_write {
                        if let Some(mut stdin_pipe) = child.stdin.take() {
                            use std::io::Write;
                            stdin_pipe.write_all(content.as_bytes())?;
                        }
                    }

                    if !is_last {
                        prev_stdout = Some(std::process::Stdio::from(child.stdout.take().unwrap()));
                    }
                    child_processes.push(child);
                }
                _ => {
                    let handler = self
                        .handlers
                        .iter()
                        .find(|h| h.can_handle(command))
                        .ok_or_else(|| anyhow::anyhow!("No handler found for builtin"))?;

                    let output = handler.handle(command)?;

                    if is_last {
                        return Ok(output);
                    } else {
                        prev_output_buffer = Some(output);

                        prev_stdout = None;
                    }
                }
            }
        }

        if let Some(mut last_child) = child_processes.pop() {
            last_child.wait()?;
        }

        for mut child in child_processes {
            let _ = child.wait();
        }

        Ok(String::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
