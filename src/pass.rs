use std::fs::{self, write};
use std::io;

use std::io::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_pass(pass: &str) -> u64 {
    let mut s = DefaultHasher::new();
    pass.hash(&mut s);
    s.finish()
}

pub fn set_password(pass: &str) -> io::Result<()> {
    // let mut file = get_password_file()?;
    // file.write(pass.as_bytes())?;
    let mut file = fs::File::create("res/passfile.txt")?;
    file.write_all(pass.as_bytes())
}

fn get_password_file() -> io::Result<fs::File> {
    fs::File::open("res/passfile.txt")
}

pub fn get_password() -> io::Result<String> {
    let mut file = get_password_file()?;
    let mut content = String::new();
    let s = file.read_to_string(&mut content)?;
    Ok(content)
}


pub fn _dep_set_password(pass: &str) -> io::Result<()> {
    let mut file = get_password_file()?;
    println!("pass?");
    println!("{}", pass);
    file.write_all(pass.as_bytes())
}
