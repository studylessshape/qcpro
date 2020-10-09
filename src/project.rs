use std::env;
use std::fs::{self, DirEntry, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

/// Create new directory and init project
pub fn new_project(directory: String) -> Result<String, io::Error> {
    match fs::create_dir(directory.clone()) {
        Ok(()) => init_project(directory.clone()),
        Err(e) => Err(e),
    }
}

/// Initialize project
/// Create two directories name of `include` and `src`
/// Create file `main.cpp` in directoy `src`
pub fn init_project(directory: String) -> Result<String, io::Error> {
    //Judge the directory is current directory or '.'
    let mut project_directoy = {
        if directory.clone() == String::from(".") {
            String::from(match env::current_dir()?.to_str() {
                Some(s) => s,
                None => ".",
            })
        } else {
            directory.clone()
        }
    };

    //Get the last directory name
    let mut last_directory: String = String::new();
    if project_directoy.contains('\\') || project_directoy.contains('/') {
        loop {
            if project_directoy.len() < 1 {
                if last_directory.len() < 1 {
                    last_directory.push_str("default");
                }
                break;
            }
            let s = project_directoy.pop().unwrap();
            if s == '\\' || s == '/' {
                break;
            } else {
                last_directory.insert(0, s);
            }
        }
    } else {
        last_directory = project_directoy.clone();
    }
    //`include` directory
    let include = directory.clone() + "/include";
    fs::create_dir(include)?;

    //`src` directory
    let src = directory.clone() + "/src";
    fs::create_dir(src)?;

    //`main.cpp` file
    let main_cpp_name = directory.clone() + "/src/main.cpp";
    File::create(&main_cpp_name)?;

    //c++ code
    let code = "#include<iostream>\nint main()\n{\n    std::cout<<\"Hello, world!\"<<std::endl;\n    return 0;\n}";
    fs::write(main_cpp_name, code)?;

    //cmake file
    let cmake_file_name = directory + "/CMakeLists.txt";
    let cmake_contents = format!(
        "{}\n{}{}{}\n{}{}{}",
        "cmake_minimum_required(VERSION 3.10)",
        "project(",
        last_directory.clone(),
        ")",
        "add_executable(",
        last_directory,
        " src/main.cpp)"
    );
    fs::write(cmake_file_name, cmake_contents)?;
    Ok(last_directory)
}

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

/// Use g++ to compile the project
pub fn run_project() -> Result<String, io::Error> {
    if env::consts::OS == "windows" {
        run_win()
    } else {
        run_shell()
    }
}

fn run_shell() -> Result<String, io::Error> {
    build_project(false)?;
    let project_name = get_project_name(&String::from("CMakeLists.txt"), false).unwrap();
    Command::new("cd")
        .arg("build")
        .spawn()
        .expect("Not found `build` directory!");

    //run g++ to compile the files
    let output = Command::new("make").output().expect("Error occured!");

    if output.status.success() {
        let output = Command::new(format!("./{}", project_name)).output().expect(format!("Occured error when run {}", project_name).as_str());
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        Ok(String::from("use make to compile project"))
    } else {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("make status: {}", output.status);
        Err(io::Error::new(
            io::ErrorKind::Other,
            "make compile project occured error!",
        ))
    }
}

fn run_win() -> Result<String, io::Error> {
    //-->>Read src files<<--
    let current_dir = match env::current_dir() {
        Ok(path) => String::from(path.to_str().unwrap()),
        Err(e) => return Err(e),
    };
    let src_path = PathBuf::from(format!("{}", current_dir));
    let project_name: String = get_project_name(&current_dir, true).unwrap();
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
            let output = Command::new(format!(".\\{}", project_name))
                .output()
                .expect(format!("Occured error when run {}", project_name).as_str());
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
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

pub fn get_project_name(source: &String, is_directory: bool) -> Option<String> {
    let mut s_cp = source.clone();
    let mut project_name = String::new();
    if is_directory{
        loop {
            if s_cp.len() < 1 {
                break;
            }
            let ch = s_cp.pop().unwrap();
            if ch == '\\' || ch == '/' {
                break;
            }
            project_name.insert(0, ch);
        }
    }else{
        if let Ok(contents) = fs::read(source) {
            let contents = String::from_utf8(contents).unwrap();
            let pat = "project(";
            let fir = match qcpro_core::string_addition::first_index(&contents, pat) {
                Some(n) => n,
                None => return None,
            };
            let contents: Vec<char> = contents.chars().collect();
            for idx in fir + pat.len() + 1..contents.len() {
                if contents[idx] == ')' {
                    break;
                }
                project_name.push(contents[idx]);
            }
        }
    }

    if project_name.len() < 1 {
        return None;
    }else{
        return Some(project_name);
    }
}
