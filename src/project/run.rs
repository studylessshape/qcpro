// >>---std mod use---<<
use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::process::Command;

// >>---Self mod use---<<
use super::build;
use crate::addition::string_addition;
use crate::command::Command as QcproCommand;
/// On windows run `run_win`, another `run_shell`. They are private function, except `run_project`.
/// * Windows:
///   When use `qcpro run` on windows, not like shell, it use g++ to compile file and run the executable file. It mean qcpro don't use the project directory built to compile and run.
///   So, g++ will compile all of file end with `.cpp` and  `.c` and ignore directory `build` and `bin`(It corresponds `read_path_file`).
/// * Shell:
///   Before run executable file, it will use CMake to build and use make to compile. All of these is in function `build_project`
///   Then run by the path from project name that get by function `get_project_name`
pub fn run_project(command: QcproCommand) -> Result<String, io::Error> {
    if env::consts::OS == "windows" {
        run_win(command)
    } else {
        run_shell(command)
    }
}

fn run_shell(command: QcproCommand) -> Result<String, io::Error> {
    build::build_project(false)?;
    let project_name =
        string_addition::get_project_name(&String::from("CMakeLists.txt"), false).unwrap();
    let exit_status = Command::new(format!("./build/{}", project_name))
        .args(command.subaction)
        .status()?;
    if exit_status.success() {
        Ok(String::from("Success run"))
    } else {
        println!("\'{}\' status: {}", project_name, exit_status);
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Run \'{}\' occured error!", project_name),
        ))
    }
}

fn run_win(command: QcproCommand) -> Result<String, io::Error> {
    // Read src files
    // There are some risks if read file directly.
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
    let project_name: String = string_addition::get_project_name(&current_dir, true).unwrap();
    let s = read_path_file(&src_path, &vec![String::from("build"), String::from("bin")])?;
    // End Read

    if s.len() < 1 {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    } else {
        // Get the c/c++ files
        let build_args: Vec<String> = s
            .into_iter()
            .filter(|file| {
                let file_c = file.to_lowercase();
                if file_c.ends_with(".cpp")
                    || file_c.ends_with(".cxx")
                    || file_c.ends_with(".c")
                    || file_c.ends_with(".c++")
                    || file_c.ends_with(".cc")
                {
                    return true;
                }
                false
            })
            .collect();
        if build_args.len() < 1 {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
        match fs::create_dir("./bin") {
            Err(e) => {
                if !(e.kind() == io::ErrorKind::AlreadyExists) {
                    return Err(e);
                }
            }
            _ => {}
        }
        //run g++ to compile the files
        let g_compile_status = Command::new("clang++")
            .arg("-o")
            .arg(format!("./bin/{}", project_name.clone()))
            .args(build_args)
            .status()?;

        if g_compile_status.success() {
            let project_executable_status = Command::new("cmd")
                .args(vec!["/C", &format!(".\\bin\\{}.exe", project_name)])
                .args(command.subaction)
                .status()?;
            if !project_executable_status.success() {
                return Err(io::Error::new(io::ErrorKind::Other, "Run Error!"));
            }
            Ok(String::from("use g++ to compile project and run it"))
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "clang compile project occured error!",
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
    let path_cp = String::from(path.to_str().unwrap());
    for pa in ignore {
        if pa.eq(&last_path) || pa.eq(&path_cp) {
            result = true;
            break;
        }
    }
    result
}
