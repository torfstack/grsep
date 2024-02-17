pub trait Source {
    fn next_line(&self, buf: &mut String) -> isize;
}

pub struct File {
    file: std::fs::File,
}

impl Source for File {
    fn next_line(&self, buf: &mut String) -> isize {
        let line = std::io::BufRead::read_line(&mut std::io::BufReader::new(&self.file), buf);
        match line {
            Ok(n) => n as isize,
            Err(_) => -1
        }
    }
}

impl File {
    pub fn new(filename: &str) -> File {
        File {
            file: std::fs::File::open(filename).unwrap(),
        }
    }
}

pub struct Stdin {
    stdin: std::io::Stdin,
}

impl Source for Stdin {
    fn next_line(&self, buf: &mut String) -> isize {
        let line = self.stdin.read_line(buf);
        match line {
            Ok(n) => n as isize,
            Err(_) => -1
        }
    }
}

impl Stdin {
    pub fn new() -> Stdin {
        Stdin {
            stdin: std::io::stdin(),
        }
    }
}