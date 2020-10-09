use std::{io, env};
use std::fs::{self, File};

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
                break;
            }
            let s = project_directoy.pop().unwrap();
            if s == '\\' || s == '/' {
                break;
            } else {
                last_directory.insert(0, s);
            }
        }
        if last_directory.len() < 1 {
            last_directory.push_str("default");
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