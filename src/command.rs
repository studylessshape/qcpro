use std::io;

use super::help;
use super::project::{build, initialize, new, run};
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
    pub options: Vec<String>,
}

pub enum QcproReturnKind {
    Success(String),
    Other,
}

/// Generate struct Command by args. Split action, subaction, option to different field
impl Command {
    pub fn new(mut args: std::env::Args) -> Result<Command, &'static str> {
        args.next();
        let mut other: Vec<String> = Vec::new();
        let mut options: Vec<String> = Vec::new();
        for arg in args {
            if arg.chars().next().unwrap() == '-' {
                options.push(arg);
            } else {
                other.push(arg);
            }
        }
        let mut subaction: Vec<String> = Vec::new();
        let mut action: String = String::new();
        if other.len() > 0 {
            action = other[0].clone();

            let mut iter = other.into_iter();
            iter.next();
            subaction = iter.collect();
        }
        Ok(Command {
            action,
            subaction,
            options,
        })
    }
}

/// relate project
impl Command {
    pub fn run_command(self) -> Result<QcproReturnKind, io::Error> {
        if self.options.len() > 0 {
            let help_s = vec![String::from("--help"), String::from("-h")];
            let version_s = vec![String::from("--version"), String::from("-v")];
            for option in &self.options {
                if option.eq(&help_s[0]) || option.eq(&help_s[1]) {
                    print_help();
                } else if option.eq(&version_s[0]) || option.eq(&version_s[1]) {
                    help::print_version();
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Unkown option \'{}\'", option),
                    ));
                }
            }
            return Ok(QcproReturnKind::Other);
        }

        if self.action.len() < 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "To few arguments",
            ));
        }

        let _new_s = String::from("new");
        let _init_s = String::from("init");
        let _build_s = String::from("build");
        let _run_s = String::from("run");

        if self.action == _new_s {
            let dir: String = match self.subaction.len() {
                0 => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "To few arguments",
                    ))
                }
                _ => self.subaction[0].clone(),
            };
            match new::new_project(dir) {
                Ok(s) => Ok(QcproReturnKind::Success(format!("Create project {}", s))),
                Err(e) => Err(e),
            }
        } else if self.action == _init_s {
            let dir: String = match self.subaction.len() {
                0 => String::from("."),
                _ => self.subaction[0].clone(),
            };
            match initialize::init_project(dir) {
                Ok(s) => Ok(QcproReturnKind::Success(format!(
                    "Initialize project {}",
                    s
                ))),
                Err(e) => Err(e),
            }
        } else if self.action == _build_s {
            match build::build_project(true) {
                Ok(s) => Ok(QcproReturnKind::Success(s)),
                Err(e) => Err(e),
            }
        } else if self.action == _run_s {
            match run::run_project(self) {
                Ok(_) => Ok(QcproReturnKind::Other),
                Err(e) => Err(e),
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid Input! Please use 'qcpro --help' to print help",
            ))
        }
    }
}

/// If printing help occured error, it will panic
pub fn print_help() {
    if let Err(e) = help::print_help() {
        panic!(e);
    }
}
