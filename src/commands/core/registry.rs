use crate::commands::{
    cd_command_handler::ChangeDirCommandHandler, command_handler::CommandHandler,
    echo_command_handler::EchoCommandHandler, history_command_handler::HistoryCommandHandler,
    history_state::HistoryState, locate_command_handler::LocateCommandHandler,
    pwd_command_handler::PwdCommandHandler, type_command_handler::TypeCommandHandler,
    unspecified_command_handler::UnspecifiedCommandHandler,
};
use std::sync::{Arc, Mutex};

pub type Handler = Arc<dyn CommandHandler>;

pub struct CommandRegistry {
    handlers: Vec<Handler>,
}

impl CommandRegistry {
    pub fn new(state: Arc<Mutex<HistoryState>>) -> Self {
        let handlers: Vec<Handler> = vec![
            Arc::new(EchoCommandHandler),
            Arc::new(TypeCommandHandler),
            Arc::new(LocateCommandHandler),
            Arc::new(UnspecifiedCommandHandler),
            Arc::new(PwdCommandHandler),
            Arc::new(ChangeDirCommandHandler),
            Arc::new(HistoryCommandHandler::new(state)),
        ];

        Self { handlers }
    }

    pub fn handlers(&self) -> Vec<Handler> {
        self.handlers.clone()
    }
}
