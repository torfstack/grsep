use colored::Colorize;

pub fn pretty_print(text: &str, pattern_len: usize, indices: Vec<usize>) {
    if indices.is_empty() {
        println!("{}", text);
        return;
    }

    print!("{}", &text[0..indices[0]]);
    print!("{}", &text[indices[0]..indices[0]+pattern_len].red());

    indices[1..].iter()
        .enumerate()
        .for_each(|(i, x)| {
            print!("{}", &text[indices[i]+1..*x]);
            print!("{}", &text[*x..*x+pattern_len].red());
        });
    print!("{}\n", &text[indices[indices.len()-1]+pattern_len..]);
}