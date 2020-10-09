use std::io::{self, Write};
use std::process::Command;

/// Use cmake to simply build project
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
        Ok(String::from("use cmake build!"))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "CMake build occured error!",
        ))
    }
}
