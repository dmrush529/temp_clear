//utility to empty temp folder on windows
//intended to be user-generic, so anyone could use
use whoami::username;

fn main() {
    let username = username();
    println!("Hello, {}!", username);
}
