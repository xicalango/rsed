
use std::io::BufRead;
use std::result;

use Result;

use buffer::Buffer;

use action::Action;

#[derive(Debug)]
pub enum Mode {
    Command,
    Insert
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

    pub fn get_input(&self) -> Result<Action> {
        unimplemented!()
    }

}
