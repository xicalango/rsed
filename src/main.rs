
extern crate rsed;

use std::env;

fn main() {
    run(env::args()).unwrap();
}

fn run(mut args: env::Args) -> rsed::Result<()> {

    let mut rsed = rsed::Rsed::new();

    if let Some(p) = args.nth(1) {
        try!(rsed.read_file(p));
    }

    rsed.main_loop();

    Ok(())
}

