use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::net::IpAddr;

#[allow(unused)]
fn main() {
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", error);
                }
            },
        };
    }

    {
        let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    };

    {
        // let greeting_file = File::open("hello.txt").unwrap();
        let greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in this project");
    }

    {
        let home: IpAddr = "127.0.0.1"
            .parse()
            .expect("hardcoded Ip address should be valid");
    }
}

#[allow(unused)]
fn read_uname_from_file() -> Result<String, io::Error> {
    let uname_file_res = File::open("hello.txt");

    let mut uname_file = match uname_file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut uname = String::new();

    match uname_file.read_to_string(&mut uname) {
        Ok(_) => Ok(uname),
        Err(e) => Err(e),
    }
}

#[allow(unused)]
fn read_uname_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(unused)]
fn read_uname_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

#[allow(unused)]
fn read_username_from_file_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

#[allow(unused)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[allow(unused)]
fn funky_fun() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

#[allow(unused)]
fn panic_not_to_panic() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("guess value must be between 1 and 100, got {}", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
