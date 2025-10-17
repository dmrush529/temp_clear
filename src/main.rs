//utility to empty temp folder on windows
//intended to be user-generic, so anyone could use
use whoami::username;
use std::fs::{read_dir, remove_dir_all, remove_file, DirEntry};

//main function
fn main() {
    let username = username();
    let path = format!("C:\\Users\\{}\\AppData\\Local\\Temp", username);
    //read files into variable
    let files = read_dir(path.clone()).expect("Failed to read directory")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .collect::<Vec<DirEntry>>();

    //read the directories into the same
    let directories = read_dir(path.clone()).expect("Failed to read directory")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .collect::<Vec<DirEntry>>();

    println!("only files: {:?} \n", files);
    println!("directories: {:?} \n", directories);
}
