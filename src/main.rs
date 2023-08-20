use std::env;
use std::fs;

fn main() {

    // Read in and store command line arguments
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // Read file contents
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file");
    
    println!("With text:\n{contents}");
}
