extern crate regex;

pub mod buffer;
pub mod ui;
pub mod action;
pub mod pos;

use std::result;
use std::env::Args;
use std::fs::File;
use std::path::Path;
use std::io::{
    BufReader,
    stdin
};

use std::convert;

pub type Result<T> = result::Result<T, Error>; // TODO update to better error

#[derive(Debug)]
pub enum ErrorType {
    Unknown,
    ParseError
}

#[derive(Debug)]
pub struct Error {
    msg: String,
    error: ErrorType
}

impl convert::From<regex::Error> for Error {
    fn from(e: regex::Error) -> Error {
        Error::detailed(ErrorType::ParseError, format!("{}", e))
    }
}

impl Error {
    pub fn unknown(msg: &str) -> Error {
        Error {
            msg: msg.to_string(),
            error: ErrorType::Unknown
        }
    }

    pub fn new(error: ErrorType) -> Error {
        Error { 
            msg: String::new(), 
            error: error 
        }
    }

    pub fn detailed(error: ErrorType, msg: String) -> Error {
        Error {
            msg: msg,
            error: error
        }
    }
}

struct InputInformation {
    position: usize,
    input_buffer: buffer::Buffer
}

#[derive(Debug)]
struct Rsed {
    current_buffer: buffer::Buffer,
    input_buffer: Option<buffer::Buffer>,
    ui: ui::Ui,
}

impl Rsed {

    pub fn new() -> Rsed {
        Rsed {
            current_buffer: buffer::Buffer::new(),
            input_buffer: Option::None,
            ui: ui::Ui::new()
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Rsed> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let buffer = buffer::Buffer::from_buf_read(reader);

        Ok(Rsed {
            current_buffer: buffer,
            input_buffer: Option::None,
            ui: ui::Ui::new()
        })
    }

    pub fn main_loop(&mut self) {

        let mut running = true;

        while(running) {
            let input = self.ui.get_input();

        }
    }

}


pub fn run(mut args: Args) -> Result<()> {

    let path = args.nth(1).expect("fail");

    let rsed = try!(Rsed::from_path(path));

    rsed.ui.display(&rsed.current_buffer);

    Ok(())
}
