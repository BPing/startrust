use error_chain::*;
use std::fs::File;
use crate::error_chain_demo::ErrorKind::Duple;


error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "Error during IO"];
    }
    errors {
        Single {
            description("MyError!")
            display("Single Error")
        }
        Duple(t: String) {
            description("MyError!")
            display("Dutple {} Error", t)
        }
    }
}

fn read_file() -> Result<()> {
    let _file = File::open("unknown_file.txt")?;
    std::result::Result::Ok(())
}

fn duple_error_func() -> Result<()> {
    Err(Duple(String::from("d")).into())
}

pub fn error_chain_func() {
    match read_file() {
        std::result::Result::Ok(_) => println!("No Error"),
        std::result::Result::Err(e) => println!("{}", e),
    }
    match duple_error_func() {
        std::result::Result::Ok(_) => println!("No Error"),
        std::result::Result::Err(e) => println!("{}", e),
    }
}