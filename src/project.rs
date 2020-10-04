use std::{io::{self, Write},env};
use std::fs::{self, File};
use std::process::Command;
/// create new directory the init project
pub fn new_project(directory: String) -> Result<String, io::Error> {
    match fs::create_dir(directory.clone()) {
        Ok(()) => {
            match init_project(directory.clone())
            {
                Ok(kind) => Ok(kind),
                Err(e)=>Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

/// initialize project
/// create two directories name of `include` and `src`
/// create file `main.cpp` in directoy `src`
pub fn init_project(directory: String) -> Result<String, io::Error> {
    let mut project_directoy = {
        if directory.clone() == String::from("."){
            String::from(
                match env::current_dir()?.to_str() {
                    Some(s)=>s,
                    None=>"."
                }
            )
        }else{
            directory.clone()
        }
    };
    let mut last_directory : String = String::new();
    if project_directoy.contains('\\') || project_directoy.contains('/') {
        loop {
            let s = project_directoy.pop().unwrap();
            if s == '\\' || s == '/' {
                break;
            }else{
                last_directory.insert(0, s);
            }
        }
    }else{
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
    let cmake_contents = format!("{}\n{}{}{}\n{}{}{}",
                                "cmake_minimum_required(VERSION 3.10)",
                                "project(", last_directory.clone(), ")",
                                "add_executable(", last_directory, " src/main.cpp)");
    fs::write(cmake_file_name, cmake_contents)?;
    Ok(last_directory)
}

/// Use cmake to simply build project
pub fn cmake_build_project(target : &Vec<String>) -> Result<String, io::Error> {
    let mut source_path : String = String::new();
    let mut build_path : String = String::new();
    if target.len() < 2 {
        source_path.push('.');
        build_path.push_str("./build");
    }else{
        source_path.push_str(&target[0]);
        build_path.push_str(&target[1]);
    }
    let output= match Command::new("cmake")
                        .args(&["-S", &source_path, "-B", &build_path])
                        .output(){
                            Ok(out)=>out,
                            Err(e)=>return Err(e),
                        };

    // let os_str_output = output.stdout;
    // println!("{}", String::from_utf8_lossy(&os_str_output));
    
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    println!("status: {}", output.status);
    Ok(String::from("use cmake build!"))
}