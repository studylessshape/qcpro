use qcpro::{Command, QcproReturnKind};
use std::env;
use std::process;
fn main() {
    match Command::new(env::args()) {
        Ok(com) => match com.run_command() {
            Ok(skind) => {
                if let QcproReturnKind::Success(s) = skind {
                    println!("{}", s);
                }
            }
            Err(e) => {
                println!("Error of application: {:#}", e);
            }
        }
        Err(s) => {
            println!("Error of application: {}", s);
            process::exit(1);
        }
    };
}
