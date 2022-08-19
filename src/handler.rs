use crate::command;
use crate::conf;
use prettytable::{cell, format, row, Table};
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
            eprintln!("[CAM ERROR]: {}", err);
            process::exit(1);
        });
        conf::create_camrc().unwrap_or_else(|err| {
            eprintln!("[CAM ERROR]: {}", err);
            process::exit(1);
        });
        let env_path = conf::get_env_path().unwrap();
        println!(
            "[CAM INFO]: camer initialization succeeded, please run `source {}`",
            env_path
        )
    } else {
        println!("[CAM INFO]: camer has been initialized")
    }
}

pub fn ls() {
    if conf::exist_camrc() {
        let file_content = conf::read_camrc().unwrap_or_else(|err| {
            eprintln!("[CAM ERROR]: {}", err);
            process::exit(1);
        });
        let reg = Regex::new(r"alias ([0-9a-zA-Z_]*)='([\S ]*)'").unwrap();

        let mut alias_table = Table::new();
        alias_table.set_format(*format::consts::FORMAT_NO_BORDER);
        alias_table.set_titles(row![
            Frbc -> "Alias",
            Frbc -> "Command"
        ]);
        for caps in reg.captures_iter(file_content.as_str()) {
            alias_table.add_row(row![
                Fmc -> caps.get(1).unwrap().as_str(),
                Fmc -> caps.get(2).unwrap().as_str()
            ]);
        }
        alias_table.printstd();
    } else {
        println!("[CAM INFO]: camer is not initialized, please run 'camer init'");
    }
}

pub fn add(args: &command::Add) {
    if conf::exist_camrc() {
        if check_name_exist(&args.name) {
            println!("[CAM INFO]: {} alias already exists", args.name);
            process::exit(1);
        }
        let alias_shell = "alias ".to_owned() + &args.name + "='" + &args.shell + "'";
        conf::append_camrc(alias_shell).unwrap_or_else(|err| {
            eprintln!("[CAM ERROR]: {}", err);
            process::exit(1);
        });
        let env_path = conf::get_env_path().unwrap();
        println!(
            "[CAM INFO]: {} added successfully, please run `source {}`",
            args.name, env_path
        );
    } else {
        println!("[CAM INFO]: camer is not initialized, please run 'camer init'");
    }
}

pub fn remove(args: &command::Remove) {
    if conf::exist_camrc() {
        if check_name_exist(&args.name) {
            let file_content = conf::read_camrc().unwrap_or_else(|err| {
                eprintln!("[CAM ERROR]: {}", err);
                process::exit(1);
            });
            let reg = Regex::new(r"alias ([0-9a-zA-Z_]*)='([0-9a-zA-Z_ ]*)'").unwrap();
            let remove_alias = reg
                .captures_iter(file_content.as_str())
                .filter(|cap| cap.get(1).unwrap().as_str() == args.name)
                .map(|cap| cap.get(0).unwrap().as_str())
                .collect::<Vec<_>>();
            for alias in remove_alias {
                conf::remove_camrc(alias).unwrap_or_else(|err| {
                    eprintln!("[CAM ERROR]: {}", err);
                    process::exit(1);
                })
            }
            let env_path = conf::get_env_path().unwrap();
            println!(
                "[CAM INFO]: remove {} alias succeeded, please run `source {}`",
                args.name, env_path
            );
        }
    } else {
        println!("[CAM INFO]: camer is not initialized, please run 'camer init'");
    }
}

fn check_name_exist(name: &String) -> bool {
    let file_content = conf::read_camrc().unwrap_or_else(|err| {
        eprintln!("[CAM ERROR]: {}", err);
        process::exit(1);
    });
    let reg = Regex::new(r"alias ([0-9a-zA-Z_]*)='([0-9a-zA-Z_ ]*)'").unwrap();
    let filter_res = reg
        .captures_iter(file_content.as_str())
        .map(|cap| cap.get(1).unwrap().as_str())
        .filter(|&cap_name| cap_name == name);
    filter_res.count() > 0
}
