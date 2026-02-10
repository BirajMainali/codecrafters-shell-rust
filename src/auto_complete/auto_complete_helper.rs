use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::cell::{Cell, RefCell};
use std::io::{self, Write};

pub struct AutoCompleteHelper {
    commands: Vec<String>,
    last_input: RefCell<String>,
    tab_count: Cell<usize>,
}

impl AutoCompleteHelper {
    pub fn new(commands: Vec<String>) -> Self {
        AutoCompleteHelper {
            commands,
            last_input: RefCell::new(String::new()),
            tab_count: Cell::new(0),
        }
    }
}

impl Helper for AutoCompleteHelper {}
impl Hinter for AutoCompleteHelper {
    type Hint = String;
}
impl Validator for AutoCompleteHelper {}
impl Highlighter for AutoCompleteHelper {}

impl Completer for AutoCompleteHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let mut matches: Vec<String> = self
            .commands
            .iter()
            .filter(|c| c.starts_with(line))
            .cloned()
            .collect();

        matches.sort();
        matches.dedup();

        if matches.is_empty() {
            return Ok((0, vec![]));
        }

        if matches.len() == 1 {
            let cmd = format!("{} ", matches[0]);
            return Ok((
                0,
                vec![Pair {
                    display: cmd.clone(),
                    replacement: cmd,
                }],
            ));
        }

        let lcp = longest_common_prefix(&matches);
        if lcp.len() > line.len() {
            return Ok((
                0,
                vec![Pair {
                    display: lcp.clone(),
                    replacement: lcp,
                }],
            ));
        }

        if *self.last_input.borrow() != line {
            self.tab_count.set(0);
            *self.last_input.borrow_mut() = line.to_owned();
        }

        self.tab_count.set(self.tab_count.get() + 1);

        if self.tab_count.get() == 1 {
            print!("\x07");
        } else {
            println!("\r\n{}", matches.join("  "));
            print!("$ {}", line);
            self.tab_count.set(0);
        }
        io::stdout().flush().unwrap();

        Ok((0, vec![]))
    }
}

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut lcp = strs[0].clone();
    for s in strs.iter().skip(1) {
        while !s.starts_with(&lcp) && !lcp.is_empty() {
            lcp.pop();
        }
    }
    lcp
}
