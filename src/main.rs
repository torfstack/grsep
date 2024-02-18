mod input;
mod grep;
mod output;

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
    let args: Args = Args::parse();
    let pattern = grep::Pattern::new(&args.pattern);
    let filename = &args.file;

    let mut input: Box<dyn input::Source> = match filename {
        Some(f) => Box::new(input::File::new(f)),
        None => Box::new(input::Stdin::new()),
    };

    let buf = &mut String::new();
    let mut i = 0;
    while input.next_line(buf) > -1 {
        let trimmed = buf.trim();
        let (res, found) = grep::grep(&trimmed, &pattern);
        if found {
            print!("Line {}: ", i);
            output::pretty_print(&trimmed, args.pattern.len(), res);
        };
        i += 1;
        buf.clear();
    }
}