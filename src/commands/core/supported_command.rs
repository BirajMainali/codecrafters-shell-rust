#[derive(Debug, Clone)]
pub enum SupportedCommand {
    Echo {
        args: Vec<String>,
    },
    Exit,
    TypeCheck {
        cmd: String,
    },
    LocateExecutable {
        cmd: String,
    },
    Unspecified {
        cmd: String,
        args: Vec<String>,
    },
    NoArgument,
    Pwd,
    ChangeDir {
        path: String,
    },
    History {
        action: HistoryAction,
    },
    Redirection {
        kind: RedirectionKind,
        inner_cmd: Box<SupportedCommand>,
        output_file: String,
    },
    Pipeline {
        commands: Box<Vec<SupportedCommand>>,
    },
}

#[derive(Debug, Clone)]
pub enum HistoryAction {
    Display { limit: Option<usize> },
    Read { path: String },
    Write { path: String },
    Append { path: String },
}

impl SupportedCommand {
    pub fn commands() -> Vec<String> {
        vec![
            "echo".to_string(),
            "exit".to_string(),
            "type".to_string(),
            "pwd".to_string(),
            "history".to_string(),
        ]
    }

    pub fn supported(cmd: &String) -> bool {
        SupportedCommand::commands().contains(cmd)
    }
}

#[derive(Debug, Clone)]
pub enum RedirectionKind {
    OverwriteOnlySuccess,
    OverwriteOnlyError,
    AppendOnlySuccess,
    AppendOnlyError,
}
