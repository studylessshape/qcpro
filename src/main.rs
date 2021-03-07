use qcpro::{Command};
use std::{env, process};
use ansi_term::Colour;
use std::io::{stderr, Write};
fn main() {
    match Command::new(env::args()) {
        Ok(com) => match com.run_command() {
            Ok(skind) => {
                if let Some(s) = skind {
                    match env::consts::OS {
                        "windows" => println!("Success: {}", s),
                        _=>println!("{}: {}",Colour::Green.bold().paint("Success"), s),
                    }
                }
            }
            Err(e) => {
                match env::consts::OS {
                    "windows" => {
                        let err_output = format!("Error: {:#}", e);
                        stderr().write_all(err_output.as_bytes()).unwrap();
                    },
                    _=> {
                        let err_output = format!("{}: {:#}", Colour::Red.bold().paint("Error"), e);
                        stderr().write_all(err_output.as_bytes()).unwrap();
                    },
                }
                process::exit(1);
            }
        }
        Err(s) => {
            match env::consts::OS {
                "windows" => {
                    let err_output = format!("Error: {:#}", s);
                    stderr().write_all(err_output.as_bytes()).unwrap();
                },
                _=> {
                    let err_output = format!("{}: {:#}", Colour::Red.bold().paint("Error"), s);
                    stderr().write_all(err_output.as_bytes()).unwrap()
                },
            }
            process::exit(2);
        }
    };
}