use std::env;
use std::fs;
use std::process;

fn main() {
    let args : Vec<String> = env::args().collect();
    let command = match parse_args(args) {
        Ok(com)=>com,
        Err(s)=> {
            println!("{}", s);
            process::exit(1);
        },
    };
    println!("{:?}", command);
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

fn parse_args(args : Vec<String>)->Result<Command, String> {
    if args.len() <= 2 {
        Err(String::from("At least two arguments are required!"))
    }else{
        Ok(Command::new(args[1].clone(), args[2].clone()))
    }
}

