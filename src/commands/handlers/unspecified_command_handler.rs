use crate::commands::command_handler::CommandHandler;
use crate::commands::path_helper::PathHelper;
use crate::commands::supported_command::SupportedCommand;

pub struct UnspecifiedCommandHandler;

impl CommandHandler for UnspecifiedCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::Unspecified { .. })
    }

    /// Handles a standalone external command execution.
    /// Processes are spawned using .spawn() and waited for using .wait_with_output()
    /// to avoid buffering large or streaming outputs entirely into memory.
    fn handle(&self, cmd: &SupportedCommand) -> anyhow::Result<String> {
        let SupportedCommand::Unspecified { cmd, args } = cmd else {
            anyhow::bail!("Unsupported command passed to UnspecifiedCommandHandler");
        };

        let mut child = self.spawn_process(
            cmd,
            args,
            std::process::Stdio::inherit(),
            std::process::Stdio::inherit(),
            std::process::Stdio::inherit(),
        )?;

        child.wait()?;

        Ok(String::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

use std::os::unix::process::CommandExt;

impl UnspecifiedCommandHandler {
    /// Spawns an external process with specific Stdio configuration for redirection.
    pub fn spawn_process(
        &self,
        cmd: &str,
        args: &[String],
        stdin: std::process::Stdio,
        stdout: std::process::Stdio,
        stderr: std::process::Stdio,
    ) -> anyhow::Result<std::process::Child> {
        let Some(path) = PathHelper::find_executable(cmd) else {
            anyhow::bail!("{}: command not found", cmd);
        };

        std::process::Command::new(&path)
            .arg0(cmd) // Senior Note: Set argv[0] to shorthand name as expected by convention
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|e| anyhow::anyhow!("failed to spawn {}: {}", cmd, e))
    }
}
