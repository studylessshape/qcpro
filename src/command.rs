use std::io;

use super::help;
use super::project::{new, initialize, build, run};
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
    Print,
}

impl Command {

    /// use iter to create struct Command
    pub fn new(mut args: std::env::Args) -> Result<Command, &'static str> {
        args.next();
        let action = match args.next() {
            Some(arg)=>arg,
            None=>return Err("Failed to read arguments"),
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
        let _build_s = String::from("build");
        let _run_s = String::from("run");
        let _help_s = String::from("--help");
        let _version_s = String::from("--version");
        if self.action == _help_s || self.subaction.contains(&_help_s) {
            print_help();
            Ok(QcproReturnKind::Print)
        }else if self.action == _version_s || self.subaction.contains(&_version_s){
            help::print_version();
            Ok(QcproReturnKind::Print)
        }else if self.action == _new_s {
            let dir : String = match subact.len(){
                0=> return Err(io::Error::new(io::ErrorKind::InvalidInput, "To few arguments")),
                _=> subact[0].clone(),
            };
            match new::new_project(dir) {
                Ok(s) => Ok(QcproReturnKind::Success(format!("new {}", s))),
                Err(e)=> Err(e),
            }
        } else if self.action == _init_s {
            let dir : String = match subact.len(){
                0=> String::from("."),
                _=> subact[0].clone(),
            };
            match initialize::init_project(dir) {
                Ok(s) => Ok(QcproReturnKind::Success(format!("init {}", s))),
                Err(e)=> Err(e),
            }
        } else if self.action == _build_s{
            match build::build_project(true) {
                Ok(s) => Ok(QcproReturnKind::Success(s)),
                Err(e)=> Err(e),
            }
        }else if self.action == _run_s {
            match run::run_project() {
                Ok(s)=>Ok(QcproReturnKind::Success(s)),
                Err(e)=> Err(e),
            }
        }else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Input! Please use 'qcpro --help' to print help"))
        }
    }
    
    
}

/// If printing help occured error, it will panic 
pub fn print_help() {
    if let Err(e) = help::print_help() {
        panic!(e);
    }
}
