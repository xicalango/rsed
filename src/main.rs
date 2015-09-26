
extern crate rsed;

use std::env;

use rsed::pos::*;

fn test_parse(s: &str) {
    let range = s.parse::<Range>().unwrap();
    println!("{:?}", range);
}

fn main() {

    test_parse("%");
    test_parse("1");
    test_parse("3,$");
    test_parse(".");
    test_parse("$");
    test_parse("$,$");
    test_parse("1,3");
    test_parse("1,.");

    //rsed::run(env::args()).ok().expect("fail");
}
