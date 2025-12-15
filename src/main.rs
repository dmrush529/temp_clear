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

    //create counters for how many files and directories are deleted
    let mut files_deleted = 0;
    let mut directories_deleted = 0;

    //remove the files
    for file in &files {
        let f = remove_file(file.path());
        match f {
            Ok(_) => {files_deleted+=1;},
            Err(error) => {eprintln!("Failed to remove file: {}", error)}
        }
    }

    //remove the directories
    for directory in &directories {
        let dir = remove_dir_all(directory.path());
        match dir {
            Ok(_) => {directories_deleted+=1;},
            Err(error) => {eprintln!("Failed to delete directory: {}", error)}
        }
    }

    //store the number of files and directories deleted in a variable
    let deleted = format!("{} files removed out of {}, {} directories removed out of {}", files_deleted, &files.len(), directories_deleted, &directories.len());
    //acknowledge it's done (open a terminal for this)
    println!("{}", deleted);
    eprintln!("press Enter to exit");
    std::io::stdin().read_line(&mut String::new()).expect("Failed to read line");

}
