//utility to empty temp folder on windows
//intended to be user-generic, so anyone could use
use whoami::username;
use std::fs::{read_dir, remove_file};

//main function to get full file path to start with
fn main() {
    let username = username();
    let path = format!("C:\\Users\\{}\\AppData\\Local\\Temp", username);

    let files = read_dir(path).unwrap();
    for file in files {
        println!("{}", file.unwrap().path().display());
    }
}
