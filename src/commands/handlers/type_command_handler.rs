use crate::commands::{command_handler::CommandHandler, supported_command::SupportedCommand};

pub struct TypeCommandHandler;

impl CommandHandler for TypeCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::TypeCheck { cmd: _ })
    }

    fn handle(&self, cmd: &SupportedCommand) -> anyhow::Result<String> {
        match cmd {
            SupportedCommand::TypeCheck { cmd } if cmd == "history" => {
                Ok(format!("{} is a shell builtin\n", cmd))
            }
            SupportedCommand::TypeCheck { cmd } => Ok(format!("{} is a shell builtin\n", cmd)),
            _ => anyhow::bail!("Unsupported command passed to TypeCommandHandler"),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
