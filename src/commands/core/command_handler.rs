use crate::commands::supported_command::SupportedCommand;
use anyhow::Result;
/// Trait defining the behavior of a shell command handler.
pub trait CommandHandler: Send + Sync {
    /// Returns true if this handler can process the given command.
    fn can_handle(&self, cmd: &SupportedCommand) -> bool;

    /// Executes the command and returns the output as a string.
    fn handle(&self, cmd: &SupportedCommand) -> Result<String>;

    /// Allows downcasting for specific handler implementations when needed.
    fn as_any(&self) -> &dyn std::any::Any;
}
