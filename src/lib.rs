extern crate regex;

pub mod buffer;
pub mod ui;
pub mod pos;
pub mod cmd;

use std::result;
use std::env::Args;
use std::fs::File;
use std::path::Path;
use std::io;

use std::convert;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorType {
    Unknown,
    ParseError,
    IoError(io::Error)
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

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::new(ErrorType::IoError(e))
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
    running: bool,
}

impl Rsed {

    pub fn new() -> Rsed {
        Rsed {
            current_buffer: buffer::Buffer::new(),
            input_buffer: Option::None,
            ui: ui::Ui::new(),
            running: true
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Rsed> {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let buffer = buffer::Buffer::from_buf_read(reader);

        Ok(Rsed {
            current_buffer: buffer,
            input_buffer: Option::None,
            ui: ui::Ui::new(),
            running: true
        })
    }

    pub fn main_loop(&mut self) {

        let mut stdin = io::stdin();

        while(self.running) {
            let action_result = self.ui.get_input(&mut stdin);

            match action_result {
                Ok(action) => self.handle_action(action),
                Err(_) => println!("?")
            };
        }
    }

    fn handle_action(&mut self, action: ui::Action) {
        match action {
            ui::Action::Command(command::Command::Quit) => self.running = false,
            rest => println!("{:?}", rest),
        };
    }

}


pub fn run(mut args: Args) -> Result<()> {

    let path = args.nth(1).expect("fail");

    let mut rsed = try!(Rsed::from_path(path));

    rsed.main_loop();

    Ok(())
}
