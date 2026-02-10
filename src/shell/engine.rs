use anyhow::Result;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use crate::auto_complete::auto_complete_helper::AutoCompleteHelper;
use crate::commands::command_handler::CommandHandler;
use crate::commands::history_state::HistoryState;
use crate::commands::path_helper::PathHelper;
use crate::commands::pipeline_command_handler::PipelineCommandHandler;
use crate::commands::redirection_command_handler::RedirectionCommandHandler;
use crate::commands::supported_command::SupportedCommand;
use crate::parsing::command_parser::CommandParser;

/// A type alias for a thread-safe, shared command handler.
type Handler = Arc<dyn CommandHandler>;

/// The main Shell engine responsible for the REPL loop, history management,
/// and command dispatching.
pub struct Shell {
    /// Registered command handlers.
    handlers: Vec<Handler>,
    /// Shared in-memory history state.
    state: Arc<Mutex<HistoryState>>,
    /// Path to the history file, typically from the `HISTFILE` environment variable.
    history_file: Option<String>,
}

impl Shell {
    pub fn new(handlers: Vec<Handler>, state: Arc<Mutex<HistoryState>>) -> Self {
        let history_file = std::env::var("HISTFILE").ok();

        Self {
            handlers,
            state,
            history_file,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut rl = self.setup_readline()?;
        self.load_history(&mut rl);

        loop {
            io::stdout().flush().unwrap();

            let input = match rl.readline("$ ") {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                    continue;
                }
            };

            if input.trim().is_empty() {
                continue;
            }

            self.record_history(&mut rl, &input)?;

            let command = CommandParser::parse(input.trim());

            if let Some(should_exit) = self.dispatch_command(&command)? {
                if should_exit {
                    break;
                }
            }
        }

        self.save_history()?;
        Ok(())
    }

    fn setup_readline(
        &self,
    ) -> Result<Editor<AutoCompleteHelper, rustyline::history::FileHistory>> {
        let mut rl = Editor::new()?;

        let executables = PathHelper::get_all_executables().unwrap_or_default();
        let supported_cmds = SupportedCommand::commands();
        let all_commands: Vec<String> = executables
            .into_iter()
            .chain(supported_cmds.into_iter())
            .collect();

        let helper = AutoCompleteHelper::new(all_commands);
        rl.set_helper(Some(helper));

        Ok(rl)
    }

    fn load_history(&self, rl: &mut Editor<AutoCompleteHelper, rustyline::history::FileHistory>) {
        if let Some(ref path) = self.history_file {
            if std::path::Path::new(path).exists() {
                if let Ok(file) = std::fs::File::open(path) {
                    use std::io::BufRead;
                    let reader = std::io::BufReader::new(file);
                    let mut s = self.state.lock().unwrap();
                    for line in reader.lines().flatten() {
                        if !line.trim().is_empty() {
                            let _ = rl.add_history_entry(&line);
                            s.commands.push(line);
                        }
                    }
                    s.last_appended_index = s.commands.len();
                }
            }
        }
    }

    fn record_history(
        &self,
        rl: &mut Editor<AutoCompleteHelper, rustyline::history::FileHistory>,
        input: &str,
    ) -> Result<()> {
        rl.add_history_entry(input)?;
        self.state
            .lock()
            .unwrap()
            .commands
            .push(input.trim().to_string());
        Ok(())
    }

    fn dispatch_command(&self, command: &SupportedCommand) -> Result<Option<bool>> {
        match command {
            SupportedCommand::Exit => return Ok(Some(true)),
            SupportedCommand::NoArgument => return Ok(None),
            SupportedCommand::Redirection { .. } => {
                let handler = RedirectionCommandHandler::new(self.handlers.clone());
                self.execute_handler(&handler, command)?;
            }
            SupportedCommand::Pipeline { .. } => {
                let handler = PipelineCommandHandler::new(self.handlers.clone());
                self.execute_handler(&handler, command)?;
            }
            _ => {
                if let Some(handler) = self.handlers.iter().find(|h| h.can_handle(command)) {
                    self.execute_handler(handler.as_ref(), command)?;
                } else {
                    eprintln!("No handler found for the command");
                }
            }
        }
        Ok(None)
    }

    fn execute_handler(
        &self,
        handler: &dyn CommandHandler,
        command: &SupportedCommand,
    ) -> Result<()> {
        match handler.handle(command) {
            Ok(output) if !output.is_empty() => {
                println!("{}", output.trim_end_matches('\n'));
            }
            Err(err) => {
                println!("{}", err.to_string().trim_end_matches('\n'));
            }
            _ => {}
        }
        Ok(())
    }

    fn save_history(&self) -> Result<()> {
        if let Some(ref path) = self.history_file {
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(path)?;

            let s = self.state.lock().unwrap();
            let start = s.last_appended_index;
            for i in start..s.commands.len() {
                writeln!(file, "{}", s.commands[i])?;
            }
        }
        Ok(())
    }
}
