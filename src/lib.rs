mod project;
pub mod config;

use std::io;
pub use config::help;
///struct Command with two parameters
/// action:
///     new    create new directory to create project
///     init   initialize project with directory existing
/// subaction:
///     directory name
///
/// if action or subaction is string "--help", application will print command help on cmd or shell
#[derive(Debug)]
pub struct Command {
    pub action: String,
    pub subaction: Vec<String>,
}

pub enum QcproReturnKind {
    Success(String),
    PrintHelp,
}

impl Command {
    ///use iter to create struct Command
    pub fn new(mut args: std::env::Args) -> Result<Command, &'static str> {
        args.next();
        let action = match args.next() {
            Some(s) => s,
            None => return Err("Failed to read arguments"),
        };

        let subaction: Vec<String> = args.collect();
        Ok(Command { action, subaction })
    }
}

/// relate project
impl Command {
    pub fn run_command(&self) -> Result<QcproReturnKind, io::Error> {
        let subact = self.subaction.clone();
        let _new_s = String::from("new");
        let _init_s = String::from("init");
        let _help_s = String::from("--help");
        let _cmake_s = String::from("cmake");
        if self.action == _new_s {
            let dir : String = match subact.len(){
                0=> return Err(io::Error::from(io::ErrorKind::InvalidInput)),
                _=> subact[0].clone(),
            };
            match project::new_project(dir) {
                Ok(s) => Ok(QcproReturnKind::Success(format!("new {}", s))),
                Err(e)=> Err(e),
            }
        } else if self.action == _init_s {
            let dir : String = match subact.len(){
                0=> String::from("."),
                _=> subact[0].clone(),
            };
            match project::init_project(dir) {
                Ok(s) => Ok(QcproReturnKind::Success(format!("init {}", s))),
                Err(e)=> Err(e),
            }
        } else if self.action == _help_s || self.subaction.contains(&String::from("--help")) {
            print_help();
            Ok(QcproReturnKind::PrintHelp)
        }else if self.action == _cmake_s{
            match project::cmake_build_project(&self.subaction) {
                Ok(s) => Ok(QcproReturnKind::Success(s)),
                Err(e)=> Err(e),
            }
        }else {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
    
    
}

pub fn print_help() {
    match help::print_help() {
        Err(e) => panic!(e),
        _=>{},
    }
}
