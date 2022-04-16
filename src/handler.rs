use crate::command;
use crate::conf;
use regex::Regex;
use std::process;

pub fn clean() {
    if conf::exist_camrc() {
        conf::delete_camrc().unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
        conf::unmount_camrc_env().unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
    }
    println!("[CAM Info]: Cam cleanup succeeded")
}

pub fn init() {
    if !conf::exist_camrc() {
        conf::mount_camrc_env().unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
        conf::create_camrc().unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
        println!("[CAM Info]: cam initialization succeeded")
    } else {
        println!("[CAM Error]: cam has been initialized")
    }
}

pub fn ls() {
    if conf::exist_camrc() {
        let file_content = conf::read_camrc().unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
        let reg = Regex::new(r"alias ([0-9a-zA-Z_]*)='([0-9a-zA-Z_ ]*)'").unwrap();
        println!("");
        for caps in reg.captures_iter(file_content.as_str()) {
            println!(
                "     {:-<12} {}",
                caps.get(1).unwrap().as_str().to_owned() + " ",
                caps.get(2).unwrap().as_str(),
            )
        }
        println!("");
    } else {
        println!("[CAM Error]: cam is not initialized, please run 'cam init'");
    }
}

pub fn add(args: &command::Add) {
    let alias_shell = "alias ".to_owned() + &args.name + "='" + &args.shell + "'";
    if conf::exist_camrc() {
        conf::append_camrc(alias_shell).unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
        println!("[CAM Info]: {} added successfully", args.name);
    } else {
        println!("[CAM Error]: cam is not initialized, please run 'cam init'");
    }
}

pub fn remove(args: &command::Remove) {
    if conf::exist_camrc() {
        let _file_content = conf::read_camrc().unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
        // TODO:filter
    } else {
        println!("[CAM Error]: cam is not initialized, please run 'cam init'");
    }
}
