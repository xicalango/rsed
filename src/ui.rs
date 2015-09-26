
use std::io;

use Result;

use buffer::Buffer;
use cmd;

#[derive(Debug)]
pub enum Mode {
    Command,
    Insert
}

#[derive(Debug)]
pub struct Ui {
    mode: Mode
}

#[derive(Debug)]
pub enum Action {
    Command(cmd::Cmd),
    Insert(String),
    InsertEnd,
}

impl Ui {

    pub fn new() -> Ui {
        Ui {
            mode: Mode::Command
        }
    }

    pub fn display(&self, buffer: &Buffer, range: (usize, usize)) {
        let (from, to) = range;

        for line in buffer.get_lines( from, to ) {
            println!("{}", line);
        }
    }

    pub fn get_input(&self, stdin: &io::Stdin) -> Result<Action> {

        let mut input = String::new();

        try!(stdin.read_line(&mut input));

        let trim_input = input.trim_right();

        match self.mode {
            Mode::Command => Ok(Action::Command(try!(trim_input.parse()))),
            Mode::Insert => Ok(Action::Insert(trim_input.to_string()))
        }

    }

}
