
use std::io;
use std::ops;

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
    Numbered,
    LineEndings
}

pub struct DisplayModel<'a> {
    buffer: &'a Buffer,
    range: ops::Range<usize>,
    option: PrintOption
}

impl <'a> DisplayModel<'a> {
    pub fn new(buffer: &'a Buffer, range: ops::Range<usize>, option: PrintOption) -> DisplayModel {
        DisplayModel {
            buffer: buffer,
            range: range,
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
        
        for (line_nr, line) in model.buffer.get_lines( &model.range ).iter().enumerate() {

            let output = match model.option {
                PrintOption::Normal => format!("{}", line),
                PrintOption::Numbered => format!("{}\t{}", line_nr + model.range.start + 1, line),
                PrintOption::LineEndings => format!("{}$", line)
            };

            println!("{}", output);
        }

    }

    pub fn get_input(&self, stdin: &io::Stdin) -> Result<Action> {

        let mut input = String::new();

        let len = try!(stdin.read_line(&mut input));

        if len == 0 {
            return Ok(Action::Command(cmd::Cmd::Quit));
        }

        let trim_input = input.trim_right();

        match self.mode {
            Mode::Command => Ok(Action::Command(try!(trim_input.parse()))),
            Mode::Insert => Ok(Action::Insert(trim_input.to_string()))
        }

    }

}
