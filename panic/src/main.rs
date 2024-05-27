use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // this case just returns panic error
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file")
            }, 
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}