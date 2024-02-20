mod input;
mod grep;
mod output;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <pattern> [file]", args[0]);
        std::process::exit(1);
    }
    let pattern = grep::Pattern::new(&args[1]);
    let mut input = if args.len() == 2 {
        Box::new(input::Stdin::new()) as Box<dyn input::Source>
    } else {
        Box::new(input::File::new(&args[2])) as Box<dyn input::Source>
    };

    let mut buf = String::new();
    while input.next_line(&mut buf) != -1 {
        let trimmed = buf.trim();
        let (res, found) = grep::grep(&trimmed, &pattern);
        if found {
            output::pretty_print(&trimmed, pattern.length(), res);
        }
        buf.clear();
    }
}
