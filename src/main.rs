use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // we will pass args as cargo run {query} {filename}
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");
}
