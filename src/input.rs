use std::io::{BufRead, Lines, StdinLock};

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
    pub fn new(filename: &str) -> Option<File> {
        let file = std::fs::File::open(filename);
        match file {
            Ok(f) => {
                Some(File {
                    reader: std::io::BufReader::new(f),
                })
            }
            Err(_) => None,
        }
    }
}

pub struct Stdin {
    reader: Lines<StdinLock<'static>>
}

impl Source for Stdin {
    fn next_line(&mut self, buf: &mut String) -> isize {
        match self.reader.next() {
            Some(Ok(line)) => {
                buf.push_str(&line);
                line.len() as isize
            }
            _ => -1,
        }
    }
}

impl Stdin {
    pub fn new() -> Stdin {
        Stdin {
            reader: std::io::stdin().lines(),
        }
    }
}
