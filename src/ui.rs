
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

#[derive(Debug)]
pub enum PrintOption {
    Normal,
    Numbered
}

pub struct DisplayModel<'a> {
    buffer: &'a Buffer,
    from: usize,
    to: usize,
    option: PrintOption
}

impl <'a> DisplayModel<'a> {
    pub fn new(buffer: &'a Buffer, from: usize, to: usize, option: PrintOption) -> DisplayModel {
        DisplayModel {
            buffer: buffer,
            from: from,
            to: to,
            option: option
        }
    }
}


impl Ui {

    pub fn new() -> Ui {
        Ui {
            mode: Mode::Command
        }
    }

    pub fn display<'a>(&self, model: DisplayModel<'a>) {
        
        for (line_nr, line) in model.buffer.get_lines( model.from, model.to ).iter().enumerate() {

            let output = match model.option {
                PrintOption::Normal => format!("{}", line),
                PrintOption::Numbered => format!("{}\t{}", line_nr + model.from + 1, line)
            };

            println!("{}", output);
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
