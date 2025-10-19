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

    //remove the files
    for file in &files {
        remove_file(file.path()).expect("Failed to remove file");
    }

    //remove the directories
    for directory in &directories {
        remove_dir_all(directory.path()).expect("Failed to remove directory");
    }

    //store the number of files and directories deleted in a variable
    let deleted = format!("{} files removed, {} directories removed", &files.len(), &directories.len());
    //acknowledge it's done (open a terminal for this)
    println!("{}", deleted);
    eprintln!("press Enter to exit");
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");

}
