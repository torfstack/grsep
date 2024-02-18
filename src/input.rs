use std::io::BufRead;

pub trait Source {
    fn next_line(&mut self, buf: &mut String ) -> isize;
}

pub struct File {
    lines: Vec<String>,
    at: usize,
}

impl Source for File {
    fn next_line(&mut self, buf: &mut String) -> isize {
        if self.at < self.lines.len() {
            let line = &self.lines[self.at];
            buf.push_str(line);
            self.at += 1;
            line.len() as isize
        } else {
            -1
        }
    }
}

impl File {
    pub fn new(filename: &str) -> File {
        let file = std::fs::File::open(filename);
        match file {
            Ok(f) => {
                let reader = std::io::BufReader::new(f);
                let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
                File {
                    lines,
                    at: 0,
                }
            }
            Err(_) => panic!("File not found")
        }
    }
}

pub struct Stdin {
    lines: Vec<String>,
    at: usize,
}

impl Source for Stdin {
    fn next_line(&mut self, buf: &mut String) -> isize {
        if self.at < self.lines.len() {
            let line = &self.lines[self.at];
            buf.push_str(line);
            self.at += 1;
            line.len() as isize
        } else {
            -1
        }
    }
}

impl Stdin {
    pub fn new() -> Stdin {
        let reader = std::io::stdin();
        let lines: Vec<String> = reader.lock().lines().map(|l| l.unwrap()).collect();
        Stdin {
            lines,
            at: 0,
        }
    }
}