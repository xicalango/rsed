
use std::iter::FromIterator;
use std::io::BufRead;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Buffer {
    lines: Vec<String>,
    cached_num_lines: usize
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
            cached_num_lines: 0 as usize
        }
    }

    pub fn from_buf_read<R: BufRead + Sized> (buf_read: R) -> Buffer {

        let lines = buf_read.lines();

        let lines_vec = lines.map(|r| r.unwrap()).collect::<Vec<String>>();
        let cached_len = lines_vec.len();

        Buffer {
            lines: lines_vec,
            cached_num_lines: cached_len
        }

    }

    pub fn insert_lines<I: IntoIterator<Item=String>>(&mut self, pos: usize, insert: I) {
        let added_lines = self.lines.insert_all(pos, insert);
        self.cached_num_lines += added_lines;
    }

    pub fn insert_buffer(&mut self, pos: usize, buffer: Buffer) {
        self.lines.insert_all(pos, buffer.lines);
        self.cached_num_lines += buffer.cached_num_lines;
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
        self.cached_num_lines += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.cached_num_lines == 0
    }

    pub fn len(&self) -> usize {
        self.cached_num_lines
    }

    fn is_out_of_bounds(&self, pos: usize) -> bool {
        pos > self.len()
    }

    pub fn get_lines(&self, begin: usize, end: usize) -> &[String] {
        assert!(! self.is_out_of_bounds( begin ), format!("Out of bounds: {}/0", begin) );
        assert!(! self.is_out_of_bounds( end ), format!("Out of bounds: {}/{}", end, self.len()) );


        &self.lines[ begin .. end ]
    }

}


