//utility to empty temp folder on windows
//intended to be user-generic, so anyone could use
use whoami::username;

//main function to get full file path to start with
fn main() {
    let username = username();
    let path = format!("C:\\Users\\{}\\AppData\\Local\\Temp", username);

    println!("full user path to temp folder: {}", path);
}
