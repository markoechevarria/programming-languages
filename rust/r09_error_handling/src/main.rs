use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file = File::open("file.txt");

    let _handled_file_1 = match greeting_file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
               ErrorKind::NotFound => match File::create("file.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problems creating the file {e:?}")
               },
                _ => {
                    panic!("Problems opening the file: {error:?}");
                }
            } 
        }
    };

    let _handled_file_2 = File::open("file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("file.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {error:?}");
            })
        } else {
            panic!("Problems opening the file: {error:?}");
        }
    });

    let _handled_file_3 = File::open("file.txt").unwrap();
    let _handled_file_4 = File::open("file.txt").expect("file.txt shoudl be included in this project");

    let _handled_file_5 = read_username_from_file_1();
    let _handled_file_6 = read_username_from_file_2();
    let _handled_file_7 = read_username_from_file_3();
    let _handled_file_8 = read_username_from_file_4();
}

use std::io::{self, Read};
use std::fs;

fn read_username_from_file_1() -> Result<String, io::Error> {
    let username_file_result = File::open("file.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("file.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("file.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
