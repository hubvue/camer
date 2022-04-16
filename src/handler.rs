use crate::command;
use crate::conf;
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
        let _file_content = conf::read_camrc().unwrap_or_else(|err| {
            eprintln!("[CAM Error]: {}", err);
            process::exit(1);
        });
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
