use crate::commands::{command_handler::CommandHandler, supported_command::SupportedCommand};
use crate::core::utils::PathHelper;
use std::{env, io};

pub struct ChangeDirCommandHandler;

impl CommandHandler for ChangeDirCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::ChangeDir { .. })
    }

    fn handle(&self, cmd: &SupportedCommand) -> anyhow::Result<String> {
        let SupportedCommand::ChangeDir { path } = cmd else {
            anyhow::bail!("Unsupported command passed to ChangeDirCommandHandler");
        };

        let mut path = path.clone();

        if path == "~" {
            path = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        }

        if !PathHelper::path_exists(&path) {
            anyhow::bail!("cd: {}: No such file or directory", path);
        }

        match PathHelper::change_dir(&path) {
            Ok(_) => Ok(String::new()),

            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    anyhow::bail!("cd: {}: No such file or directory", path)
                }
                io::ErrorKind::PermissionDenied => {
                    anyhow::bail!("cd: {}: Permission denied", path)
                }
                _ => {
                    anyhow::bail!("cd: {}: {}", path, e)
                }
            },
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
