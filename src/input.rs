use std::io::BufRead;

pub trait Source {
    fn next_line(&mut self, buf: &mut String) -> isize;
}

pub struct File {
    reader: std::io::BufReader<std::fs::File>,
}

impl Source for File {
    fn next_line(&mut self, buf: &mut String) -> isize {
        match self.reader.read_line(buf) {
            Ok(n) => {
                if n == 0 {
                    -1
                } else {
                    n as isize
                }
            }
            Err(_) => -1,
        }
    }
}

impl File {
    pub fn new(filename: &str) -> File {
        let file = std::fs::File::open(filename);
        match file {
            Ok(f) => {
                File {
                    reader: std::io::BufReader::new(f),
                }
            }
            Err(_) => panic!("File not found")
        }
    }
}
