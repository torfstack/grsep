mod input;
mod grep;
mod output;

use std::{env, io};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    #[arg(short = 'f', long = "file")]
    file: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <pattern>", args[0]);
        std::process::exit(1);
    }
    let pattern = grep::Pattern::new(&args[1]);

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (res, found) = grep::grep(&line, &pattern);
        if found {
            output::pretty_print(&line, pattern.length(), res);
        }
    }
}