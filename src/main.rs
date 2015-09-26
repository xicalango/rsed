
extern crate rsed;

use std::env;

use rsed::command::*;

fn test_parse(s: &str) {
    let range = s.parse::<Command>();
    println!("{:?}", range);
}

fn main() {

    test_parse("%?");
    test_parse("1i");
    test_parse("3,$i");
    test_parse(".i");
    test_parse("$i");
    test_parse("$,$?");
    test_parse("1,3?");
    test_parse("1,.?");
    test_parse("q");
    test_parse("1");
    test_parse("%");
    test_parse("");

    //rsed::run(env::args()).ok().expect("fail");
}
