use std::env; // used for reading command line arguments

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
