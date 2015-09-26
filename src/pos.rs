

use std::str;
use regex::Regex;

use {
    Result,
    Error,
    ErrorType
};

#[derive(Debug)]
pub enum Pos {
    Line(usize),
    Current,
    End
}

impl str::FromStr for Pos {
    type Err = Error;
    fn from_str(s: &str) -> Result<Pos> {
        let re = Regex::new(r"(?P<current>\.)|(?P<end>\$)|(?P<line>\d+)").unwrap();
 
        if let Some(captures) = re.captures(s) {

            if let Some(_) = captures.name("current") {
                return Ok(Pos::Current);
            }

            if let Some(_) = captures.name("end") {
                return Ok(Pos::End);
            }

            if let Some(line) = captures.name("line") {
                let line_nr: usize = match line.parse() {
                    Ok(n) => n,
                    Err(_) => return Err(Error::new(ErrorType::ParseError))
                };

                return Ok(Pos::Line(line_nr));
            }
        }

        Err(Error::detailed(ErrorType::ParseError, s.to_string()))
    }
}

#[derive(Debug)]
pub enum Range {
    Line(Pos),
    Range(Pos, Pos)
}

impl str::FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Range> {
        let re = Regex::new(r"(?P<all>%)|(?P<first>[.$0-9]+)(,(?P<second>[.$0-9]+))?").unwrap();

        if let Some(captures) = re.captures(s) {
            if let Some(_) = captures.name("all") {
                return Ok( Range::Range( Pos::Line(0), Pos::End ) );
            }

            if let Some(first) = captures.name("first") {

                let first_pos = try!(first.parse::<Pos>());

                if let Some(second) = captures.name("second") {
                    let second_pos = try!(second.parse::<Pos>());

                    return Ok(Range::Range( first_pos, second_pos ));
                }

                return Ok(Range::Line( first_pos ));
            }
        }

        Err(Error::detailed(ErrorType::ParseError, s.to_string()))
    }
}


