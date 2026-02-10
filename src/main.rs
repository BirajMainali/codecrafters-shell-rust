use codecrafters_shell::{
    commands::{history_state::HistoryState, registry::CommandRegistry},
    shell::Shell,
};
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    let state = Arc::new(Mutex::new(HistoryState::default()));
    let registry = CommandRegistry::new(state.clone());
    let mut shell = Shell::new(registry.handlers(), state);
    shell.run()?;

    Ok(())
}
