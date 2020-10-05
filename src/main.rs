use qcpro::{Command, QcproReturnKind};
use std::{process,env};
use ansi_term::Colour;
fn main() {
    match Command::new(env::args()) {
        Ok(com) => match com.run_command() {
            Ok(skind) => {
                if let QcproReturnKind::Success(s) = skind {
                    println!("{}: {}",Colour::Green.bold().paint("Success"), s);
                }
            }
            Err(e) => {
                println!("{}: {:#}", Colour::Red.bold().paint("Error"), e);
                process::exit(1);
            }
        }
        Err(s) => {
            println!("{}: {}",Colour::Red.bold().paint("Error"), s);
            process::exit(1);
        }
    };
}
