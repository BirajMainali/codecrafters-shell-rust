use crate::commands::{command_handler::CommandHandler, supported_command::SupportedCommand};

pub struct EchoCommandHandler;

impl CommandHandler for EchoCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::Echo { .. })
    }

    fn handle(&self, cmd: &SupportedCommand) -> anyhow::Result<String> {
        let SupportedCommand::Echo { args } = cmd else {
            anyhow::bail!("Unsupported command passed to EchoCommandHandler");
        };

        let mut output = args.join(" ");
        output.push('\n');
        Ok(output)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
