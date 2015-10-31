
use std::iter::FromIterator;
use std::io::{
    BufRead,
    Write
};
use std::vec::IntoIter;
use std::ops;


use Result;

#[derive(Debug)]
pub struct Buffer {
    lines: Vec<String>,
    cached_num_lines: usize,
    modified: bool
}

trait InsertAll {
    type Item;

    fn insert_all<I: IntoIterator<Item=Self::Item>>(&mut self, pos: usize, insert: I) -> usize;
}

impl <T> InsertAll for Vec<T> {
    type Item = T;

    fn insert_all<I: IntoIterator<Item=Self::Item>>(&mut self, pos: usize, insert: I) -> usize {
        let mut count = 0 as usize;
        for (index, item) in insert.into_iter().enumerate() {
            self.insert( pos + index, item );
            count += 1;
        }

        count
    }
}

impl FromIterator<String> for Buffer {
    fn from_iter<T>(iter: T) -> Buffer
        where T: IntoIterator<Item=String> {
            let mut buffer = Buffer::new();
            buffer.insert_lines(0, iter);
            return buffer
        }
}

impl IntoIterator for Buffer {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

impl Buffer {

    pub fn new() -> Buffer {
        Buffer { 
            lines: Vec::new(),
            cached_num_lines: 0 as usize,
            modified: false
        }
    }

    pub fn from_buf_read<R: BufRead + Sized> (buf_read: R) -> Result<Buffer> {

        let lines = buf_read.lines();

        let lines_vec = lines.map(|r| r.unwrap()).collect::<Vec<String>>();
        let cached_len = lines_vec.len();

        Ok(Buffer {
            lines: lines_vec,
            cached_num_lines: cached_len,
            modified: false
        })

    }

    pub fn insert_lines<I: IntoIterator<Item=String>>(&mut self, pos: usize, insert: I) {
        let added_lines = self.lines.insert_all(pos, insert);
        self.cached_num_lines += added_lines;
        self.modified = true;
    }

    pub fn insert_buffer(&mut self, pos: usize, buffer: Buffer) {
        self.lines.insert_all(pos, buffer.lines);
        self.cached_num_lines += buffer.cached_num_lines;
        self.modified = true;
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
        self.cached_num_lines += 1;
        self.modified = true;
    }

    pub fn delete_lines(&mut self, start: usize, end: usize) {
        for _ in start..end {
            self.lines.remove(start);
        }

        self.cached_num_lines -= end - start;
    }

    pub fn is_empty(&self) -> bool {
        self.cached_num_lines == 0
    }

    pub fn has_changes(&self) -> bool {
        self.modified
    }

    pub fn len(&self) -> usize {
        self.cached_num_lines
    }

    pub fn is_out_of_bounds(&self, pos: usize) -> bool {
        pos > self.len()
    }

    pub fn is_range_out_of_bounds(&self, range: &ops::Range<usize>) -> bool {
        self.is_out_of_bounds(range.start) || self.is_out_of_bounds(range.end)
    }

    pub fn get_lines(&self, range: &ops::Range<usize>) -> &[String] {
        assert!(! self.is_out_of_bounds( range.start ), format!("Out of bounds: {}/0", range.start) );
        assert!(! self.is_out_of_bounds( range.end ), format!("Out of bounds: {}/{}", range.end, self.len()) );


        &self.lines[ range.start .. range.end ]
    }

    pub fn write<W: Write>(&self, w:&mut W) -> Result<()> {
        
        for line in self.lines.iter() {
            try!( w.write_all( line.as_bytes() ) );
        }

        Ok(())
    }

}


