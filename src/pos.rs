use std::str;
use std::convert;
use std::ops;
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

pub trait Converter<F, T> {
    fn convert(&self, from: F) -> T;
}

static POS_RE: &'static str = r"^((?P<current>\.)|(?P<end>\$)|(?P<line>\d+))$";
static RANGE_RE: &'static str = r"^((?P<all>%{1})|(?P<first>[.$0-9]+)(,(?P<second>[.$0-9]+))?)$";

impl str::FromStr for Pos {
    type Err = Error;
    fn from_str(s: &str) -> Result<Pos> {
        let re = Regex::new(POS_RE).unwrap();
 
        if let Some(captures) = re.captures(s) {

            if let Some(_) = captures.name("current") {
                return Ok(Pos::Current);
            }

            if let Some(_) = captures.name("end") {
                return Ok(Pos::End);
            }

            if let Some(line) = captures.name("line") {
                return  match line.parse() {
                    Ok(n) => Ok(Pos::Line(n)),
                    Err(_) => Err(Error::new(ErrorType::ParseError))
                };
            }
        }

        Err(Error::detailed(ErrorType::ParseError, s.to_string()))
    }
}

impl convert::From<Range> for Pos {
    fn from(r: Range) -> Pos {
        match r {
            Range::Line(p) => p,
            Range::Range(_, p) => p
        }
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
        let re = Regex::new(RANGE_RE).unwrap();

        if let Some(captures) = re.captures(s) {
            if let Some(_) = captures.name("all") {
                return Ok( Range::Range( Pos::Line(1), Pos::End ) );
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

impl Range {

    pub fn to_range<'a, C>(&'a self, conv: &C) -> ops::Range<usize> 
        where C: Converter<&'a Pos, usize> {
        match *self {
            Range::Line(ref p) => {
                let pos = conv.convert(&p);
                ops::Range { 
                    start: pos - 1, 
                    end: pos
                }
            },

            Range::Range(ref f, ref t) => ops::Range {
                start: conv.convert(&f) - 1, 
                end: conv.convert(&t)
            }
        }
    }

    pub fn current_line() -> Range {
        Range::Line(Pos::Current)
    }

}

