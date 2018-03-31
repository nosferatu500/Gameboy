use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::borrow::Cow;
use std::str::{self, FromStr};

use cpu::Cpu;

use nom::{digit, eof, space, IResult};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Step(usize),
    Repeat,
    Exit,
}

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command(s.as_bytes()) {
            IResult::Done(_, c) => Ok(c),
            err => Err(format!("Unable to parse command: {:?}", err).into()),
        }
    }
}

pub struct Debugger {
    cpu: Cpu,
    last_command: Option<Command>,
}

impl Debugger {
    pub fn new(cpu: Cpu) -> Debugger {
        Debugger {
            cpu,
            last_command: None,
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("gb> ");
            stdout().flush().unwrap();

            let command = match (read_stdin().parse(), self.last_command) {
                (Ok(Command::Repeat), Some(c)) => Ok(c),
                (Ok(Command::Repeat), None) => Err("No last command".into()),
                (Ok(c), _) => Ok(c),
                (Err(e), _) => Err(e),
            };

            match command {
                Ok(Command::Step(count)) => self.step(count),
                Ok(Command::Exit) => break,
                Ok(Command::Repeat) => unreachable!(),
                Err(ref e) => println!("{}", e),
            }

            self.last_command = command.ok();
        }
    }

    pub fn step(&mut self, count: usize) {
        for _ in 0..count {
            self.cpu.update_ime();
            self.cpu.run_next_instruction();
        }
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}

named!(
    command<Command>,
    chain!(
        c: alt_complete!(
            step |
            exit |
            repeat) ~
            eof,
    || c)
);

named!(
    step<Command>,
    chain!(
        alt_complete!(tag!("step") | tag!("s")) ~
            count: opt!(preceded!(space, usize_parser)),
        || Command::Step(count.unwrap_or(1)))
);

named!(
    exit<Command>,
    map!(
        alt_complete!(tag!("exit") | tag!("quit") | tag!("e") | tag!("q")),
        |_| Command::Exit
    )
);

named!(repeat<Command>, value!(Command::Repeat));

named!(
    usize_parser<usize>,
    map_res!(map_res!(digit, str::from_utf8), FromStr::from_str)
);
