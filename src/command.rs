use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(Init),
    Ls(Ls),
    Add(Add),
    Remove(Remove),
    Clean(Clean),
}

/// Clean configuration file
#[derive(Args, Debug)]
pub struct Clean;

/// Initialisation of the configuration file
#[derive(Args, Debug)]
pub struct Init;

/// Show all alias set
#[derive(Args, Debug)]
pub struct Ls;

/// Add a command alias
#[derive(Args, Debug)]
pub struct Add {
    /// Alias name
    pub name: String,
    /// Alias shell command
    pub shell: String,
}

/// Delete existing alias
#[derive(Args, Debug)]
pub struct Remove {
    /// Alias name
    pub name: String,
}
