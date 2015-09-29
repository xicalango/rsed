
extern crate rsed;

use std::env;

fn main() {
    rsed::run(env::args()).unwrap();
}
