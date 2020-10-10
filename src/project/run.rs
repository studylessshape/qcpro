// >>---std mod use---<<
use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

// >>---Self mod use---<<
use super::build;
use crate::addition;
/// Use g++ to compile the project
pub fn run_project() -> Result<String, io::Error> {
    if env::consts::OS == "windows" {
        run_win()
    } else {
        run_shell()
    }
}

fn run_shell() -> Result<String, io::Error> {
    build::build_project(false)?;
    let project_name =
        addition::string::get_project_name(&String::from("CMakeLists.txt"), false).unwrap();
    let output = Command::new(format!("./build/{}", project_name)).output()?;
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    if output.status.success() {
        Ok(String::from("Success run"))
    } else {
        println!("\'{}\' status: {}", project_name, output.status);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Run \'{}\' occured error!", project_name),
        ))
    }
}

fn run_win() -> Result<String, io::Error> {
    //-->>Read src files<<--
    if let Err(_e) = fs::read("CMakeLists.txt") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Not Find file \'CMakeLists.txt\', this directory may not be a c++ project directoy",
        ));
    }
    let current_dir = match env::current_dir() {
        Ok(path) => String::from(path.to_str().unwrap()),
        Err(e) => return Err(e),
    };
    let src_path = PathBuf::from(format!("{}", current_dir));
    let project_name: String = addition::string::get_project_name(&current_dir, true).unwrap();
    let s = read_path_file(&src_path, &vec![String::from("build"), String::from("bin")])?;
    //-->>End Read<<--

    if s.len() < 1 {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    } else {
        // Get the c/c++ files
        let mut build_args: Vec<String> = Vec::new();
        for file in s {
            let file_c = file.to_lowercase();
            if file_c.ends_with(".cpp") || file_c.ends_with(".cxx") || file_c.ends_with(".c") {
                build_args.push(file);
            }
        }
        if build_args.len() < 1 {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }

        //run g++ to compile the files
        let output = Command::new("g++")
            .arg("-o")
            .arg(project_name.clone())
            .args(build_args)
            .output()
            .expect("Error occured!");

        if output.status.success() {
            Command::new("cmd")
                .args(vec!["/C", &format!(".\\{}.exe", project_name)])
                .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .output()
                .expect(&format!("Run \'{}\' occured error!", project_name));
            Ok(String::from("use g++ to compile project and run it"))
        } else {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            println!("g++ status: {}", output.status);
            Err(io::Error::new(
                io::ErrorKind::Other,
                "g++ compile project occured error!",
            ))
        }
    }
}

/// Read files recursively
fn read_path_file(path: &PathBuf, ignore: &Vec<String>) -> Result<Vec<String>, io::Error> {
    let read_dirctory = fs::read_dir(path)?;
    if is_ignore_path(&path, ignore) {
        return Err(io::Error::from(io::ErrorKind::Other));
    }
    let all_sub_path: Vec<Result<DirEntry, io::Error>> = read_dirctory.collect();

    let mut s: Vec<String> = Vec::new();

    for i in all_sub_path {
        if let Ok(i_path) = i {
            //If the path is directory, need to read the directory.Or add the path on s directly
            match fs::read_dir(i_path.path()) {
                Ok(_sub_path) => {
                    if let Ok(s_re) = read_path_file(&i_path.path(), ignore) {
                        for i in s_re {
                            s.push(i);
                        }
                    }
                }
                Err(_) => {
                    if let Some(path) = i_path.path().to_str() {
                        s.push(String::from(path));
                    }
                }
            }
        }
    }
    Ok(s)
}

pub fn is_ignore_path(path: &PathBuf, ignore: &Vec<String>) -> bool {
    let mut last_path: String = String::new();
    let mut path_cp = String::from(path.to_str().unwrap());
    loop {
        if path_cp.len() < 1 {
            break;
        }
        let ch = path_cp.pop().unwrap();
        if ch == '\\' || ch == '/' {
            break;
        }
        last_path.insert(0, ch);
    }
    if last_path.len() < 1 {
        return false;
    }
    let mut result = false;
    for pa in ignore {
        if pa.eq(&last_path) {
            result = true;
            break;
        }
    }
    result
}
