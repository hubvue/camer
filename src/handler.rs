use crate::command;

pub fn clean() {
    println!("clean");
}

pub fn init() {
    println!("init");
}

pub fn ls() {
    println!("ls");
}

pub fn add(args: &command::Add) {
    println!("name: {}", args.name);
    println!("shell: {}", args.shell);
}

pub fn remove(args: &command::Remove) {
    println!("name: {}", args.name);
}
