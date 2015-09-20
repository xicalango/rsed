
use std::io::BufRead;
use std::result;

use buffer::Buffer;


type Result<T> = result::Result<T, String>;

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

pub struct UI<'a> {
    buffer: &'a mut Buffer,
    input_buffer: Buffer,
    mode: Mode,
    running: bool
}

impl <'a> UI<'a> {

    pub fn new(buffer: &'a mut Buffer) -> UI {

        UI {
            buffer: buffer,
            input_buffer: Buffer::new(),
            mode: Mode::Command,
            running: true
        }

    }

    fn handle_input(&mut self, num_chars: usize, line: String) {
    }

    pub fn main_loop<T: BufRead>(&mut self, input: &mut T) {

        while(self.running) {
            let mut input_buffer = String::new();
            let num_chars = input.read_line(&mut input_buffer).unwrap();

            let result = match self.mode {
                Mode::Command => self.handle_command( num_chars, input_buffer ),
                Mode::Insert => self.handle_insert( num_chars, input_buffer )
            };

            match result {
                Err(e) => println!("?"),
                _ => () 
            };
        }

    }

    fn handle_command(&mut self, num_chars: usize, line: String) -> Result<()> {
        if num_chars == 0 {
            self.running = false;
            return Ok(());
        }

        let chars: Vec<char> = line.chars().collect();

        let action = try!(Action::from_char(chars[0]));

        match action {
            Action::ModeInsert => self.switch_mode(Mode::Insert),
            Action::Quit => self.running = false
        };

        return Ok(());
    }

    fn handle_insert(&mut self, num_chars: usize, line: String) -> Result<()> {
        if num_chars == 0 {
            return Ok(self.switch_mode(Mode::Command));
        }

        return Ok(());
    }

    fn switch_mode(&mut self, new_mode: Mode) {

        match self.mode {

            Mode::Insert => {
                let len = self.buffer.len();
                let 
                self.buffer.insert_buffer(len, self.input_buffer);
            },
            _ => ()
        };

        self.mode = new_mode;
    }
}


