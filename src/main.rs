use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    (query, file_path)
}
