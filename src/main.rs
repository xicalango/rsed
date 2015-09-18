
extern crate rsed;

use std::env;
use std::fs::File;
use std::io::BufReader;

use rsed::buffer::*;

fn main() {

    let args = &mut env::args();

    let path = args.nth(1).expect("no path given");

    let f = File::open(path).unwrap();

    let reader = BufReader::new(f);

    let mut buffer = Buffer::from_buf_read(reader);

    println!("{:?}", buffer);


    {
        buffer.insert_lines(0, vec!["a", "b"].iter().map(|s| s.to_string()));
    }

    println!("{:?}", buffer);
}
