use std::fs::{self};
use std::io;
use sha3::{Digest, Sha3_256};
use std::io::prelude::*;

const PASSWORD_FILE: &str = "/tmp/termilock-passfile.txt";

pub fn hash_pass(pass: &str) -> String {
    //    let mut s = DefaultHasher::new();
    //    pass.hash(&mut s);
    //    s.finish()
    let mut hasher = Sha3_256::new();

    // write input message
    hasher.update(pass.as_bytes());

    // read hash digest
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn set_password(pass: &str) -> io::Result<()> {
    // let mut file = get_password_file()?;
    // file.write(pass.as_bytes())?;
    let mut file = fs::File::create(PASSWORD_FILE)?;
    let hashed = hash_pass(pass);
    file.write_all(hashed.as_bytes())
}

fn get_password_file() -> io::Result<fs::File> {
    fs::File::open(PASSWORD_FILE)
}

pub fn get_password() -> io::Result<String> {
    let mut file = get_password_file()?;
    let mut content = String::new();
    let s = file.read_to_string(&mut content)?;
    Ok(content)
}
