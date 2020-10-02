use std::fs::{self, File};
use std::io::{self, Write};

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
        if self.action == _new_s {
            Command::new_project(subact[0].clone())
        } else if self.action == _init_s {
            Command::init_project(&subact[0])
        } else if self.action == _help_s || self.subaction.contains(&String::from("--help")) {
            print_help();
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
    
    /// create new directory the init project
    fn new_project(directory: String) -> Result<QcproReturnKind, io::Error> {
        match fs::create_dir(directory.clone()) {
            Ok(()) => {
                match Command::init_project(&directory)
                {
                    Ok(_kind) => Ok(QcproReturnKind::Success(format!(
                        "Success create project: {}",
                        directory
                    ))),
                    Err(e)=>Err(e),
                }
                
            }
            Err(e) => Err(e),
        }
    }
    
    /// initialize project
    /// create two directories name of `include` and `src`
    /// create file `main.cpp` in directoy `src`
    fn init_project(directory: &String) -> Result<QcproReturnKind, io::Error> {
        //`include` directory
        let include = directory.clone() + "/include";
        fs::create_dir(include)?;
    
        //`src` directory
        let src = directory.clone() + "/src";
        fs::create_dir(src)?;
    
        //`main.cpp` file
        let main_cpp = directory.clone() + "/src/main.cpp";
        let mut main_cpp = File::create(main_cpp)?;
    
        //c++ code
        let code = "#include<iostream>\nint main()\n{\n    std::cout<<\"Hello, world!\"<<std::endl;\n    return 0;\n}";
        let code_buf = code.as_bytes();
        main_cpp.write(code_buf)?;
        Ok(QcproReturnKind::Success(format!(
            "Success init project: {}",
            directory
        )))
    }
}

pub fn print_help() {
    println!("qcpro [action] [subaction]");
    println!("  action:");
    println!("    new    create new project name of subaction");
    println!("    init   initialize project for directory that name of subaction");
}
