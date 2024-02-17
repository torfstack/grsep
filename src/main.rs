mod input;
mod grep;

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

    let input: Box<dyn input::Source> = match filename {
        Some(f) => Box::new(input::File::new(f)),
        None => Box::new(input::Stdin::new()),
    };

    let buf = &mut String::new();
    while input.next_line(buf) > 0 {
        let (res, found) = grep::grep(&buf, &pattern);
        if found {
            println!("{:?}", res);
        }
    }
}
