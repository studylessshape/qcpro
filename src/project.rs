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
pub fn cmake_build_project(target: &Vec<String>) -> Result<String, io::Error> {
    let mut source_path: String = String::new();
    let mut build_path: String = String::new();
    if target.len() < 2 {
        source_path.push('.');
        build_path.push_str("./build");
    } else {
        source_path.push_str(&target[0]);
        build_path.push_str(&target[1]);
    }
    let output = Command::new("cmake")
        .args(&["-S", &source_path, "-B", &build_path])
        .output()?;

    // let os_str_output = output.stdout;
    // println!("{}", String::from_utf8_lossy(&os_str_output));
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    println!("CMake status: {}", output.status);
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
pub fn compile_project() -> Result<String, io::Error> {
    //-->>Read src files<<--
    let mut current_dir = match env::current_dir() {
        Ok(path) => String::from(path.to_str().unwrap()),
        Err(e) => return Err(e),
    };
    
    let src_path = PathBuf::from(format!("{}", current_dir));
    let mut project_name: String = String::new();
    loop {
        if current_dir.len() < 1 {
            break;
        }
        let ch = current_dir.pop().unwrap();
        if ch == '\\' || ch == '/' {
            break;
        }else{
            project_name.insert(0, ch);
        }
    }
    let s = read_path_file(&src_path)?;
    //-->>End Read<<--

    if s.len() < 1 {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    } else {
        // Get the c/c++ files
        let mut build_args : Vec<String> = Vec::new();
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
                             .arg(project_name)
                             .args(build_args)
                             .output()
                             .expect("Error occured!");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("g++ status: {}", output.status);
        if output.status.success() {
            Ok(String::from("use g++ to compile project"))
        }else{
            Err(io::Error::new(
                io::ErrorKind::Other,
                "g++ compile project occured error!"
            ))
        }
        
    }
}

/// Read files recursively
fn read_path_file(path: &PathBuf) -> Result<Vec<String>, io::Error> {
    let read_dirctory = fs::read_dir(path)?;
    let all_sub_path: Vec<Result<DirEntry, io::Error>> = read_dirctory.collect();

    let mut s : Vec<String> = Vec::new();

    for i in all_sub_path {
        if let Ok(i_path) = i {
            //If the path is directory, need to read the directory.Or add the path on s directly
            match fs::read_dir(i_path.path()) {
                Ok(_sub_path) => {
                    if let Ok(s_re) = read_path_file(&i_path.path()) {
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