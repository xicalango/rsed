
extern crate rsed;

use std::env;

use rsed::command::*;

fn main() {
    rsed::run(env::args()).ok().expect("fail");
}
