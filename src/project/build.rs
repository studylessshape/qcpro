use std::io::{self, Write};
use std::process::Command;
use std::env;

/// Use cmake to simply build project
/// Input bool to control while print the cmake output on cmd of shell
pub fn build_project(is_output: bool) -> Result<String, io::Error> {
    let source_path: String = String::from(".");
    let build_path: String = String::from("./build");
    let output = Command::new("cmake")
        .args(&["-S", &source_path, "-B", &build_path])
        .output()?;
    if is_output {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("CMake status: {}", output.status);
    }
    if output.status.success() {
        if !env::consts::OS.eq_ignore_ascii_case("windows") {
            Command::new("make").arg("-C").arg("build").output()?;
        }
        Ok(String::from("use cmake build!"))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "CMake build occured error!",
        ))
    }
}
