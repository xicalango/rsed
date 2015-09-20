
extern crate rsed;

use std::env;
use std::fs::File;
use std::io::{
    BufReader,
    stdin
};

use rsed::buffer::*;
use rsed::ui;

fn main() {

    let args = &mut env::args();

    let path = args.nth(1).expect("no path given");

    let f = File::open(path).unwrap();

    let reader = BufReader::new(f);

    let mut buffer = Buffer::from_buf_read(reader);

    println!("{:?}", buffer);

    let mut ui = ui::UI::new(&mut buffer);

    let mut input = BufReader::new( stdin() );
    
    ui.main_loop( &mut input );
}
