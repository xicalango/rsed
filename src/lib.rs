
pub mod buffer;
pub mod ui;

use std::result;
use std::env::Args;
use std::fs::File;
use std::path::Path;
use std::io::{
    BufReader,
    stdin
};

pub type Result<T> = result::Result<T, String>; // TODO update to better error

#[derive(Debug)]
struct Rsed {
    currentBuffer: buffer::Buffer,
    inputBuffer: Option<buffer::Buffer>,
    ui: ui::Ui,
}

impl Rsed {

    pub fn new() -> Rsed {
        Rsed {
            currentBuffer: buffer::Buffer::new(),
            inputBuffer: Option::None,
            ui: ui::Ui::new()
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Rsed> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let buffer = buffer::Buffer::from_buf_read(reader);

        Ok(Rsed {
            currentBuffer: buffer,
            inputBuffer: Option::None,
            ui: ui::Ui::new()
        })
    }

}


pub fn run(mut args: Args) -> Result<()> {

    let path = args.nth(1).expect("fail");

    let rsed = try!(Rsed::from_path(path));

    rsed.ui.display(&rsed.currentBuffer);

    Ok(())
}
