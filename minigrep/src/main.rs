use std::env; // used for reading command line arguments
use std::fs;


fn main() {
    // .collect turns it into a vector
    // env::args() basically reads the argument from the command line
    let args: Vec<String> = env::args().collect(); 
    dbg!(&args); // print args in debug 

    let config = parse_config(args)

    println!("Searching for {} filepath", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(filepath)
        .expect("Should be able to read the file");
    
    println!("\nWith text: \n{contents}");
}

// create a way to make it so creading Config struct is more obvious

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filepath = args[2].clone();

        Config {query, filepath}
    }
}

/*
// this returns the struct defined below
fn parse_config(args: &[String]) -> (&str, &str) {
    // want to take ownership because Config struct is supposed to own its strings
    let query = args[1].clone();
    let filepath = args[2].clone();

    Config {query, filepath}
}
*/

struct Config {
    query: String,
    file_path: String,
}