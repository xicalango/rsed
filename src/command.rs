
use std::str;

use regex::Regex;

use pos;

use {
    Result,
    Error,
    ErrorType
};

#[derive(Debug)]
pub enum Command {
    EnterInsertMode(pos::Range),
    Quit,
    Debug(pos::Range),
    Jump(pos::Range),
    JumpNext,
}

static COMMAND_RE: &'static str = r"^(?P<range>[%.,$\d]+)?(?P<cmd>[a-zA-Z?])?$";

impl Command {
    fn from_char_and_range(c: char, range: pos::Range) -> Result<Command> {
        match c {
            'i' => Ok(Command::EnterInsertMode(range)),
            'q' => Ok(Command::Quit),
            '?' => Ok(Command::Debug(range)),
            _ => Err(Error::new(ErrorType::ParseError))
        }
    }
}

impl str::FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Command> {

        if s.len() == 0 {
            return Ok(Command::JumpNext);
        }

        let re = Regex::new(COMMAND_RE).unwrap();

        if let Some(captures) = re.captures(s) {

            let cmd_range = try!(captures.name("range")
                                 .map(|r| r.parse::<pos::Range>())
                                 .unwrap_or(Ok(pos::Range::Line(pos::Pos::Current))));

            return match captures.name("cmd") {
                Some(cmd) => Command::from_char_and_range(cmd.chars().next().unwrap(), cmd_range),
                None => Ok(Command::Jump(cmd_range))
            };
        } 

        Err(Error::detailed(ErrorType::ParseError, s.to_string()))
    }
}

