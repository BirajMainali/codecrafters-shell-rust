use crate::commands::supported_command::RedirectionKind;
use crate::commands::{command_handler::CommandHandler, supported_command::SupportedCommand};
use anyhow::{bail, Result};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

pub struct RedirectionCommandHandler {
    handlers: Vec<Arc<dyn CommandHandler>>,
}

impl RedirectionCommandHandler {
    pub fn new(handlers: Vec<Arc<dyn CommandHandler>>) -> Self {
        Self { handlers }
    }

    fn redirect_output(
        &self,
        kind: &RedirectionKind,
        output_file: &str,
        stdout: String,
        stderr: String,
    ) -> Result<String> {
        let append = matches!(
            kind,
            RedirectionKind::AppendOnlySuccess | RedirectionKind::AppendOnlyError
        );

        match kind {
            RedirectionKind::OverwriteOnlySuccess | RedirectionKind::AppendOnlySuccess => {
                Self::write_to_file(output_file, &stdout, append)?;
                Ok(stderr) // return stderr to caller
            }

            RedirectionKind::OverwriteOnlyError | RedirectionKind::AppendOnlyError => {
                Self::write_to_file(output_file, &stderr, append)?;
                Ok(stdout) // return stdout to caller
            }
        }
    }

    fn write_to_file(path: &str, content: &str, append: bool) -> Result<()> {
        let path = Path::new(path);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(!append)
            .open(path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl CommandHandler for RedirectionCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::Redirection { .. })
    }

    fn handle(&self, cmd: &SupportedCommand) -> Result<String> {
        let SupportedCommand::Redirection {
            kind,
            inner_cmd,
            output_file,
        } = cmd
        else {
            bail!("Unsupported command passed to RedirectionCommandHandler");
        };

        if let SupportedCommand::Unspecified { cmd, args } = inner_cmd.as_ref() {
            let unspecified_handler = self.handlers.iter()
                .find(|h| h.can_handle(inner_cmd))
                .and_then(|h| h.as_any().downcast_ref::<crate::commands::unspecified_command_handler::UnspecifiedCommandHandler>())
                .expect("UnspecifiedCommandHandler must be available");

            let append = matches!(
                kind,
                RedirectionKind::AppendOnlySuccess | RedirectionKind::AppendOnlyError
            );
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .truncate(!append)
                .open(output_file)?;

            let is_stdout_redirection = matches!(
                kind,
                RedirectionKind::OverwriteOnlySuccess | RedirectionKind::AppendOnlySuccess
            );

            let stdout = if is_stdout_redirection {
                std::process::Stdio::from(file.try_clone()?)
            } else {
                std::process::Stdio::inherit()
            };

            let stderr = if !is_stdout_redirection {
                std::process::Stdio::from(file)
            } else {
                std::process::Stdio::inherit()
            };

            let mut child = unspecified_handler.spawn_process(
                cmd,
                args,
                std::process::Stdio::inherit(),
                stdout,
                stderr,
            )?;
            child.wait()?;

            return Ok(String::new());
        }

        let handler = self
            .handlers
            .iter()
            .find(|h| h.can_handle(inner_cmd))
            .expect("Handler must exist");

        let mut stdout = String::new();
        let mut stderr = String::new();

        match handler.handle(inner_cmd) {
            Ok(out) => {
                stdout.push_str(&out);

                if !stdout.is_empty() && !stdout.ends_with('\n') {
                    stdout.push('\n');
                }
            }
            Err(e) => {
                stderr.push_str(&format!("{}", e));
                if !stderr.is_empty() && !stderr.ends_with('\n') {
                    stderr.push('\n');
                }
            }
        }

        self.redirect_output(kind, output_file, stdout, stderr)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
