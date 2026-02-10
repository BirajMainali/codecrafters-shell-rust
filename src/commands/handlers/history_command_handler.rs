use crate::commands::command_handler::CommandHandler;
use crate::commands::history_state::SharedHistory;
use crate::commands::supported_command::{HistoryAction, SupportedCommand};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct HistoryCommandHandler {
    state: SharedHistory,
}

impl HistoryCommandHandler {
    pub fn new(state: SharedHistory) -> Self {
        Self { state }
    }
}

impl CommandHandler for HistoryCommandHandler {
    fn can_handle(&self, cmd: &SupportedCommand) -> bool {
        matches!(cmd, SupportedCommand::History { .. })
    }

    fn handle(&self, cmd: &SupportedCommand) -> anyhow::Result<String> {
        let SupportedCommand::History { action } = cmd else {
            anyhow::bail!("Unsupported command passed to HistoryCommandHandler");
        };

        match action {
            HistoryAction::Display { limit } => {
                let state = self.state.lock().unwrap();
                let mut result = String::new();
                let total_count = state.commands.len();
                let start_index = if let Some(n) = limit {
                    if *n < total_count {
                        total_count - *n
                    } else {
                        0
                    }
                } else {
                    0
                };

                for (i, command) in state.commands.iter().enumerate().skip(start_index) {
                    result.push_str(&format!("{:5}  {}\n", i + 1, command));
                }
                Ok(result)
            }
            HistoryAction::Read { path } => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let mut state = self.state.lock().unwrap();
                for line in reader.lines() {
                    let line = line?;
                    if !line.trim().is_empty() {
                        state.commands.push(line);
                    }
                }
                Ok(String::new())
            }
            HistoryAction::Write { path } => {
                let mut file = File::create(path)?;
                let state = self.state.lock().unwrap();
                for command in &state.commands {
                    writeln!(file, "{}", command)?;
                }
                Ok(String::new())
            }
            HistoryAction::Append { path } => {
                let mut file = OpenOptions::new().create(true).append(true).open(path)?;
                let mut state = self.state.lock().unwrap();
                let start = state.last_appended_index;
                let end = state.commands.len();
                for i in start..end {
                    writeln!(file, "{}", state.commands[i])?;
                }
                state.last_appended_index = end;
                Ok(String::new())
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
