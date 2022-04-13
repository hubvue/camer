use clap::Parser;
mod command;
mod handler;

fn main() {
    // parse command options
    let options = command::Options::parse();
    println!("{:?}", options);

    // match command
    match options.command {
        command::Command::Ls(_) => handler::ls(),
        command::Command::Add(ref args) => handler::add(args),
        command::Command::Remove(ref args) => handler::remove(args),
        command::Command::Init(_) => handler::init(),
        command::Command::Clean(_) => handler::clean(),
    };
}
