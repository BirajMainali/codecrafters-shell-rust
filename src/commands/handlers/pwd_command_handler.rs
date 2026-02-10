use crate::commands::command_handler::CommandHandler;
use crate::commands::path_helper::PathHelper;
use crate::commands::supported_command::SupportedCommand;

pub struct PwdCommandHandler;

impl CommandHandler for PwdCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::Pwd)
    }

    fn handle(&self, cmd: &SupportedCommand) -> anyhow::Result<String> {
        let SupportedCommand::Pwd = cmd else {
            anyhow::bail!("Unsupported command passed to PwdCommandHandler");
        };

        match PathHelper::get_current_dir() {
            Some(dir) => Ok(format!("{}\n", dir)),
            None => anyhow::bail!("pwd: unable to determine current directory"),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
