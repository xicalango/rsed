
use std::str;

use regex::Regex;

use std::convert::From;

use pos;
use ui::PrintOption;
use util::FlipResultOption;

use {
    Result,
    Error,
    ErrorType
};

#[derive(Debug)]
pub enum Cmd {
    EnterInsertMode(pos::Range),
    Quit,
    Debug(pos::Range),
    Jump(pos::Range),
    JumpNext,
    Write,
    Print(pos::Range, PrintOption),
    PrintLineNumber(pos::Range),
}

static COMMAND_RE: &'static str = r"^(?P<range>[%.,$\d]+)?(?P<cmd>[a-zA-Z?=])?$";

impl Cmd {
    fn from_parsed_data(data: ParsedData) -> Result<Cmd> {
        if data.is_empty() {
            return Ok(Cmd::JumpNext);
        }

        let range = data.range.unwrap_or_else( pos::Range::current_line );

        if let Some(c) = data.cmd_char {
            match c {
                'i' => Ok(Cmd::EnterInsertMode(range)),
                'q' => Ok(Cmd::Quit),
                'w' => Ok(Cmd::Write),
                'p' => Ok(Cmd::Print(range, PrintOption::Normal)),
                'n' => Ok(Cmd::Print(range, PrintOption::Numbered)),
                'l' => Ok(Cmd::Print(range, PrintOption::LineEndings)),
                '=' => Ok(Cmd::PrintLineNumber(range)),
                '?' => Ok(Cmd::Debug(range)),
                _ => Err(Error::new(ErrorType::ParseError))
            }

        } else {
            
            if data.arg != None {
                return Err(Error::new(ErrorType::ParseError))
            }

            Ok(Cmd::Jump(range))
        }
    }
}

struct ParsedData {
    cmd_char: Option<char>,
    range: Option<pos::Range>,
    arg: Option<String>
}

impl ParsedData {
    fn is_empty(&self) -> bool {
        match (&self.cmd_char, &self.range, &self.arg) {
            (&None, &None, &None) => true,
            _ => false
        }
    }

    fn empty() -> ParsedData {
        ParsedData {
            cmd_char: None,
            range: None,
            arg: None
        }
    }
}

impl str::FromStr for ParsedData {
    type Err = Error;

    fn from_str(s: &str) -> Result<ParsedData> {

        if s.len() == 0 {
            return Ok(ParsedData::empty());
        }

        let re = try!(Regex::new(COMMAND_RE));

        if let Some(captures) = re.captures(s) {

            let cmd_range = try!(captures.name("range").map(|r| r.parse::<pos::Range>()).flip());

            let cmd_char = captures.name("cmd").and_then(|c| c.chars().next());
            
            Ok(ParsedData {
                cmd_char: cmd_char,
                range: cmd_range,
                arg: None
            })

        } else {
            Err(Error::new(ErrorType::ParseError))
        }
    }
}

impl str::FromStr for Cmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Cmd> {

        let parsed_data = try!(s.parse::<ParsedData>());

        Cmd::from_parsed_data(parsed_data)
    }
}

