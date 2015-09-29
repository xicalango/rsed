
use std::convert::{
    From,
    Into
};


pub trait FlipResultOption<R, E> {
    fn flip(self) -> Result<Option<R>, E>;
}

impl <R, E> FlipResultOption<R, E> for Option<Result<R, E>> {
    fn flip(self) -> Result<Option<R>, E> {
        match self {
            Some(Ok(r)) => Ok(Some(r)),
            Some(Err(e)) => Err(e),
            None => Ok(None)
        }
    }
}

pub trait FlipOptionResult<R, E> {
    fn flip(self) -> Option<Result<R, E>>;
}

impl <R, E> FlipOptionResult<R, E> for Result<Option<R>, E> {
    fn flip(self) -> Option<Result<R, E>> {
        match self {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,
            Err(e) => Some(Err(e))
        }
    }
}

