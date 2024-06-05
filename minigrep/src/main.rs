use std::env; // used for reading command line arguments
use std::fs;


fn main() {
    // .collect turns it into a vector
    // env::args() basically reads the argument from the command line
    let args: Vec<String> = env::args().collect(); 
    dbg!(&args); // print args in debug 

    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for {} filepath", query);
    println!("In file {}", filepath);

    let contents = fs::read_to_string(filepath)
        .expect("Should be able to read the file");
    
    println!("\nWith text: \n{contents}");
}
