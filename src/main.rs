
extern crate rsed;

use std::env;
use std::fs::File;
use std::io::{
    BufReader,
    stdin
};

fn main() {
    rsed::run(env::args()).ok().expect("fail");
}
