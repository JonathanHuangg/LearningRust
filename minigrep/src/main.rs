use std::env; // used for reading command line arguments
use std::fs;
use std::process;

fn main() {
    // .collect turns it into a vector
    // env::args() basically reads the argument from the command line
    let args: Vec<String> = env::args().collect(); 
    dbg!(&args); // print args in debug 

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {} filepath", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    
    println!("\n With text: \n{contents}");
}

// create a way to make it so creading Config struct is more obvious

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
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