
use std::io::BufRead;
use std::result;

use buffer::Buffer;


type Result<T> = result::Result<T, String>;

#[derive(Debug)]
enum Mode {
    Command,
    Insert
}

enum Action {
    ModeInsert,
    Quit
}

impl Action {
    fn from_char(c: char) -> Result<Action> {
        match c {
            'i' => Ok(Action::ModeInsert),
            'q' => Ok(Action::Quit),
            _ => Err(format!("Invalid action: {}", c))
        }
    }
}

#[derive(Debug)]
pub struct Ui {
    mode: Mode
}


impl Ui {

    pub fn new() -> Ui {
        Ui {
            mode: Mode::Command
        }
    }

    pub fn display(&self, buffer: &Buffer) {
        println!("{:?}", buffer);
    }


}
