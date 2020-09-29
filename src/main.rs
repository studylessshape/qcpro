use std::env;
use std::fs::{self,File};
use std::process;
use std::io::{self,Write};

fn main() {
    let args : Vec<String> = env::args().collect();
    let command = match parse_command(args) {
        Ok(com)=>com,
        Err(s)=> {
            println!("{}", s);
            print_help();
            process::exit(1);
        },
    };
    match run_command(command) {
        Ok(())=>println!("Success to get project!"),
        Err(e)=>{
            println!("{:?}",e.kind());
            print_help();
        }
    }
}

#[derive(Debug)]
struct Command {
    action : String,
    subaction : String,
}

impl Command {
    fn new(action : String, subaction : String) -> Command {
        Command {
            action,
            subaction,
        }
    }
}

fn parse_command(args : Vec<String>)->Result<Command, String> {
    if args.len() <= 2 {
        Err(String::from("At least two arguments are required!"))
    }else{
        Ok(Command::new(args[1].clone(), args[2].clone()))
    }
}

fn run_command(command: Command)->Result<() ,io::Error> {
    let subact=command.subaction.clone();
    let _new_s = String::from("new");
    let _init_s = String::from("init");
    match command.action[..].to_lowercase() {
        _new_s=>new_project(subact),
        _init_s=>init_project(subact),
        _=>{
            print_help();
            process::exit(1);
        }
    }
}

fn new_project(directory:String)-> Result<(),io::Error>{
    match fs::create_dir(directory.clone()) {
        Ok(())=>init_project(directory),
        Err(e)=>Err(e),
    }
}

fn init_project(directory:String)->Result<(),io::Error>{
    //`include` directory
    let include = directory.clone()+&String::from("/include");
    fs::create_dir(include)?;

    //`src` directory
    let src = directory.clone()+&String::from("/src");
    fs::create_dir(src)?;

    //`main.cpp` file
    let main_cpp = directory+&String::from("/main.cpp");
    let mut main_cpp = File::create(main_cpp)?;

    //c++ code
    let code = "#include<iostream>\nint main()\n{\n    std::cout<<\"Hello, world!\"<<std::endl;\n    return 0;\n}";
    let code_buf = code.as_bytes();
    main_cpp.write(code_buf)?;
    Ok(())
}

fn print_help() {
    println!("qcpro [action] [subaction]");
    println!("  action:");
    println!("    new    create new project name of subaction");
    println!("    init   initialize project for directory that name of subaction");
}
