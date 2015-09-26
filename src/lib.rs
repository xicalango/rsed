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

use self::cmd::Cmd;

use self::pos::RealPos;

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
    position: pos::Range,
    input_buffer: buffer::Buffer
}

#[derive(Debug)]
pub struct Rsed {
    current_buffer: buffer::Buffer,
    input_buffer: Option<buffer::Buffer>,
    current_line: usize,
    ui: ui::Ui,
    running: bool,
}

impl Rsed {

    pub fn new() -> Rsed {
        Rsed {
            current_buffer: buffer::Buffer::new(),
            input_buffer: Option::None,
            current_line: 1,
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
            current_line: 1,
            ui: ui::Ui::new(),
            running: true
        })
    }

    pub fn main_loop(&mut self) {

        let mut stdin = io::stdin();

        while self.running {
            let action_result = self.ui.get_input(&mut stdin);

            match action_result {
                Ok(action) => self.handle_action(action),
                Err(_) => println!("?")
            };
        }
    }

    fn handle_action(&mut self, action: ui::Action) {
        match action {
            ui::Action::Command(Cmd::Quit) => self.running = false,
            ui::Action::Command(Cmd::Print(r, option)) => self.print_range(r, option),
            ui::Action::Command(Cmd::Jump(r)) => self.jump_to(r),
            rest => println!("Unimplemented: {:?}", rest),
        };
    }

    fn print_range(&self, r: pos::Range, option: ui::PrintOption) {
       let (from, to) = r.to_range_tuple::<Rsed>(self);

       let model = ui::DisplayModel::new( &self.current_buffer, from, to, option );

       self.ui.display( model );

    }

    fn jump_to(&mut self, r: pos::Range) {
        self.current_line = self.get_real_pos( &pos::Pos::from(r) );
        self.print_range( pos::Range::current_line(), ui::PrintOption::Normal);
    }

}

impl pos::RealPos for Rsed {
    fn get_real_pos(&self, pos: &pos::Pos) -> usize {
        match *pos {
            pos::Pos::Line(n) => n,
            pos::Pos::Current => self.current_line,
            pos::Pos::End => self.current_buffer.len()
        }
    }
}

pub fn run(mut args: Args) -> Result<()> {

    let path = args.nth(1).expect("fail");

    let mut rsed = try!(Rsed::from_path(path));

    rsed.main_loop();

    Ok(())
}
