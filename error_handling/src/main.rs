use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

// This error points out that we’re only allowed to use the
// '?' operator in a function that returns
// - 'Result'
// - 'Option'
// - or another type that implements 'FromResidual'.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn examples_for_panic() {
    //panic!("crash and burn");
    let v = vec![1, 2, 3];
    // v[99]; // RUST_BACKTRACE=1 cargo run

    // -> unwrap panics; with expect you could provide a message when panic happens.
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error);
            }
        },
    };
}
