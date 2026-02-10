use crate::commands::{command_handler::CommandHandler, supported_command::SupportedCommand};
use crate::core::utils::PathHelper;

pub struct LocateCommandHandler;

impl CommandHandler for LocateCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::LocateExecutable { .. })
    }

    fn handle(&self, cmd: &SupportedCommand) -> anyhow::Result<String> {
        let SupportedCommand::LocateExecutable { cmd } = cmd else {
            anyhow::bail!("Unsupported command passed to LocateCommandHandler");
        };

        match PathHelper::find_executable(cmd.as_str()) {
            Some(path) => Ok(format!("{} is {}\n", cmd, path)),
            None => {
                anyhow::bail!("{}: not found", cmd);
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
