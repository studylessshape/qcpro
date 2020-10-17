use std::env;
use std::io::{self, Write};
use std::process::Command;

use ansi_term::Colour;

/// Use cmake to simply build project
/// Input bool to control while print the cmake output on cmd of shell
/// If use qcpro on shell, it will use make to compile project after cmake
pub fn build_project(is_output: bool) -> Result<String, io::Error> {
    let source_path: String = String::from(".");
    let build_path: String = String::from("./build");
    if is_output {
        let end_status = Command::new("cmake")
            .args(&["-S", &source_path, "-B", &build_path])
            .status()?;
        if end_status.success() {
            if !env::consts::OS.eq_ignore_ascii_case("windows") {
                make_compile()?;
            }
            Ok(String::from("use cmake build!"))
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "CMake build occured error!",
            ))
        }
    } else {
        let output = Command::new("cmake")
            .args(&["-S", &source_path, "-B", &build_path])
            .output()?;
        if output.status.success() {
            if !env::consts::OS.eq_ignore_ascii_case("windows") {
                make_compile()?;
            }
            Ok(String::from("use cmake build!"))
        } else {
            io::stderr().write_all(&output.stderr).unwrap();
            Err(io::Error::new(
                io::ErrorKind::Other,
                "CMake build occured error!",
            ))
        }
    }
}

fn make_compile() -> Result<String, io::Error> {
    let output = Command::new("make").arg("-C").arg("build").output()?;
    if !output.status.success() {
        println!(
            "{}",
            Colour::Yellow.paint("===================================")
        );
        io::stderr().write_all(&output.stderr).unwrap();
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Compile occured error!",
        ));
    }
    Ok(String::from("use make compile success"))
}
