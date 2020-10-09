use qcpro::{Command, QcproReturnKind};
use std::{process,env};
use ansi_term::Colour;
fn main() {
    match Command::new(env::args()) {
        Ok(com) => match com.run_command() {
            Ok(skind) => {
                if let QcproReturnKind::Success(s) = skind {
                    match env::consts::OS {
                        "windows" => println!("Success: {}", s),
                        _=>println!("{}: {}",Colour::Green.bold().paint("Success"), s),
                    }
                }
            }
            Err(e) => {
                match env::consts::OS {
                    "windows" => println!("Error: {:#}", e),
                    _=>println!("{}: {:#}", Colour::Red.bold().paint("Error"), e),
                }
                process::exit(1);
            }
        }
        Err(s) => {
            match env::consts::OS {
                "windows" => println!("Error: {:#}", s),
                _=>println!("{}: {:#}", Colour::Red.bold().paint("Error"), s),
            }
            process::exit(1);
        }
    };
}
