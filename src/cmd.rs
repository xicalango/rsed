
use std::str;

use regex::Regex;

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
    Print(pos::Range, PrintOption),
    PrintLineNumber(pos::Range),
    Read(String),
    Write(Option<String>)
}

impl str::FromStr for Cmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Cmd> {
        try!(s.parse::<ParsedData>()).to_cmd()
    }
}

static COMMAND_RE: &'static str = r"^((?P<range>[%.,$\d]+)?(?P<cmd>[a-zA-Z?=])?( (?P<arg>.*))?)$";

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

    fn to_cmd(self) -> Result<Cmd> {
        if self.is_empty() {
            return Ok(Cmd::JumpNext);
        }

        let range = self.range.unwrap_or_else( pos::Range::current_line );

        if let Some(c) = self.cmd_char {
            match c {
                'i' => expect_no_arg(&self.arg, Cmd::EnterInsertMode(range)),
                'q' => expect_no_arg(&self.arg, Cmd::Quit),
                'p' => expect_no_arg(&self.arg, Cmd::Print(range, PrintOption::Normal)),
                'n' => expect_no_arg(&self.arg, Cmd::Print(range, PrintOption::Numbered)),
                'l' => expect_no_arg(&self.arg, Cmd::Print(range, PrintOption::LineEndings)),
                '=' => expect_no_arg(&self.arg, Cmd::PrintLineNumber(range)),
                '?' => expect_no_arg(&self.arg, Cmd::Debug(range)),
                _ => Err(Error::new(ErrorType::ParseError))
            }
        } else {
            
            if self.arg != None {
                return Err(Error::new(ErrorType::ParseError))
            }

            expect_no_arg(&self.arg, Cmd::Jump(range))
        }
    }
}

fn expect_no_arg(arg: &Option<String>, cmd: Cmd) -> Result<Cmd> {
    match *arg {
        None => Ok(cmd),
        _ => Err(Error::new(ErrorType::ParseError))
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

            let cmd_range = try!(captures.name("range").map(|r| r.parse()).flip());

            let cmd_char = captures.name("cmd").and_then(|c| c.chars().next());

            let cmd_arg = captures.name("arg").map(str::to_string);
            
            Ok(ParsedData {
                cmd_char: cmd_char,
                range: cmd_range,
                arg: cmd_arg
            })

        } else {
            Err(Error::new(ErrorType::ParseError))
        }
    }
}

