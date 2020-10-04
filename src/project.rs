use std::io;
use std::fs::{self, File};
/// create new directory the init project
pub fn new_project(directory: String) -> Result<String, io::Error> {
    match fs::create_dir(directory.clone()) {
        Ok(()) => {
            match init_project(&directory)
            {
                Ok(_kind) => Ok(format!(
                    "Success create project: {}",
                    directory
                )),
                Err(e)=>Err(e),
            }
            
        }
        Err(e) => Err(e),
    }
}

/// initialize project
/// create two directories name of `include` and `src`
/// create file `main.cpp` in directoy `src`
pub fn init_project(directory: &String) -> Result<String, io::Error> {
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
    let cmake_file_name = directory.clone() + "/CMakeLists.txt";
    let cmake_contents = format!("{}\n{}{}{}\n{}{}{}",
                                "cmake_minimum_required(VERSION 3.10)",
                                "project(", directory.clone(), ")",
                                "add_executable(", directory.clone(), " src/main.cpp)");
    fs::write(cmake_file_name, cmake_contents)?;
    Ok(format!(
        "Success init project: {}",
        directory
    ))
}