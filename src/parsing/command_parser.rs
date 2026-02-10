use crate::commands::supported_command::{HistoryAction, RedirectionKind, SupportedCommand};

pub struct CommandParser;

impl CommandParser {
    pub fn parse(input: &str) -> SupportedCommand {
        let input = input.trim();
        let input = input.replace("1>>", ">>").replace("1>", ">");
        let tokens = Self::lexer(input.as_str());
        let args = Self::build_arguments(tokens);

        if args.is_empty() {
            return SupportedCommand::NoArgument;
        }

        let pipelines: Vec<&str> = input.split('|').collect();

        if pipelines.len() > 1 {
            let mut commands = Vec::new();

            for part in pipelines {
                let tokens = Self::lexer(part.trim());
                let args = Self::build_arguments(tokens);

                if args.is_empty() {
                    return SupportedCommand::NoArgument;
                }

                let cmd = Self::build_command(&args);
                commands.push(cmd);
            }

            return SupportedCommand::Pipeline {
                commands: Box::new(commands),
            };
        }

        if let Some(pos) = args
            .iter()
            .position(|arg| matches!(arg.as_str(), ">" | "2>" | "2>>" | ">>"))
        {
            if pos + 1 < args.len() {
                let cmd_args = &args[..pos];
                let output_file = &args[pos + 1];

                let cmd = if cmd_args.is_empty() {
                    SupportedCommand::NoArgument
                } else {
                    Self::build_command(cmd_args)
                };

                let identifier = &args[pos];
                let kind = match identifier.as_str() {
                    ">" => RedirectionKind::OverwriteOnlySuccess,
                    "2>" => RedirectionKind::OverwriteOnlyError,
                    "2>>" => RedirectionKind::AppendOnlyError,
                    ">>" => RedirectionKind::AppendOnlySuccess,
                    _ => unreachable!(),
                };

                return SupportedCommand::Redirection {
                    kind,
                    inner_cmd: Box::new(cmd),
                    output_file: output_file.clone(),
                };
            }
        }

        Self::build_command(&args)
    }

    fn build_command(args: &[String]) -> SupportedCommand {
        let cmd = &args[0];
        let cmd_args = args[1..].to_vec();

        match cmd.as_str() {
            "echo" => SupportedCommand::Echo { args: cmd_args },
            "exit" => SupportedCommand::Exit,
            "type" => {
                if cmd_args.is_empty() {
                    SupportedCommand::NoArgument
                } else {
                    let arg = &cmd_args[0];
                    if SupportedCommand::supported(arg) {
                        SupportedCommand::TypeCheck { cmd: arg.clone() }
                    } else {
                        SupportedCommand::LocateExecutable { cmd: arg.clone() }
                    }
                }
            }
            "pwd" => SupportedCommand::Pwd,
            "history" => {
                let action = if let Some(flag) = cmd_args.get(0) {
                    match flag.as_str() {
                        "-r" => {
                            let path = cmd_args.get(1).cloned().unwrap_or_default();
                            HistoryAction::Read { path }
                        }
                        "-w" => {
                            let path = cmd_args.get(1).cloned().unwrap_or_default();
                            HistoryAction::Write { path }
                        }
                        "-a" => {
                            let path = cmd_args.get(1).cloned().unwrap_or_default();
                            HistoryAction::Append { path }
                        }
                        _ => {
                            let limit = flag.parse::<usize>().ok();
                            HistoryAction::Display { limit }
                        }
                    }
                } else {
                    HistoryAction::Display { limit: None }
                };
                SupportedCommand::History { action }
            }
            "cd" => SupportedCommand::ChangeDir {
                path: cmd_args.get(0).cloned().unwrap_or_default(),
            },
            _ => SupportedCommand::Unspecified {
                cmd: cmd.clone(),
                args: cmd_args,
            },
        }
    }

    fn lexer(input: &str) -> Vec<ClassifiedChar> {
        let mut tokens = Vec::new();

        for c in input.chars() {
            let kind = match c {
                '\\' => CharType::Backslash,
                '\'' => CharType::SingleQuote,
                '"' => CharType::DoubleQuote,
                ' ' | '\t' => CharType::Whitespace,
                _ => CharType::NormalChar,
            };

            tokens.push(ClassifiedChar { ch: c, kind });
        }

        tokens.reverse();
        tokens
    }

    fn build_arguments(mut stack: Vec<ClassifiedChar>) -> Vec<String> {
        let mut args = Vec::new();
        let mut current = String::new();

        let mut mode: Option<CharType> = None;

        while let Some(token) = stack.pop() {
            match token.kind {
                CharType::SingleQuote => match mode {
                    None => mode = Some(CharType::SingleQuote),
                    Some(CharType::SingleQuote) => mode = None,
                    _ => current.push('\''),
                },

                CharType::DoubleQuote => match mode {
                    None => mode = Some(CharType::DoubleQuote),
                    Some(CharType::DoubleQuote) => mode = None,
                    _ => current.push('"'),
                },

                CharType::Whitespace => {
                    if mode.is_none() {
                        if !current.is_empty() {
                            args.push(std::mem::take(&mut current));
                        }
                    } else {
                        current.push(token.ch);
                    }
                }

                CharType::Backslash => match mode {
                    Some(CharType::DoubleQuote) => {
                        if let Some(next) = stack.pop() {
                            if next.ch == '"' || next.ch == '\\' {
                                current.push(next.ch);
                            } else {
                                current.push('\\');
                                current.push(next.ch);
                            }
                        } else {
                            current.push('\\');
                        }
                    }

                    Some(CharType::SingleQuote) => {
                        current.push('\\');
                    }

                    None => {
                        if let Some(next) = stack.pop() {
                            current.push(next.ch);
                        } else {
                            current.push('\\');
                        }
                    }

                    _ => {
                        current.push('\\');
                    }
                },

                CharType::NormalChar => current.push(token.ch),
            }
        }

        if !current.is_empty() {
            args.push(current);
        }

        args
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ClassifiedChar {
    ch: char,
    kind: CharType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CharType {
    NormalChar,
    SingleQuote,
    DoubleQuote,
    Whitespace,
    Backslash,
}
