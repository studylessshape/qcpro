/// This mod is about the config of program
/// It's contains:
///     - Generate config file
///     - Read config file
///     - Init program

pub mod help{
    use std::{io,fs,env};

    /// Through reading file or creating file, get the string of help.Then print on cmd or shell 
    pub fn print_help() -> Result<(), io::Error> {
        match read_help_file() {
            Ok(s)=>println!("{}", s),
            Err(e)=> {
                if let io::ErrorKind::NotFound = e.kind(){
                    match create_help_file() {
                        Ok(s) => println!("{}", s),
                        Err(e)=> return Err(e),
                    }
                }
            }
        }
        Ok(())
    }

    /// read file name of 'help.qcpro'.If it don't exist, qcpro will create 'help.qcpro' and return `Ok(String)`
    fn read_help_file() -> Result<String, io::Error> {
        match fs::read("help.qcpro") {
            Ok(contents)=>Ok(String::from_utf8_lossy(&contents).to_string()),
            Err(e)=>Err(e),
        }
    }

    //create file name of 'help.qcpro'
    fn create_help_file() -> Result<String, io::Error> {
        let current_path = match env::current_exe(){
            Ok(path)=>path,
            Err(e)=>return Err(e),
        };
        let mut current_path = String::from(current_path.to_str().unwrap());
        loop {
            let ch = current_path.pop().unwrap();
            if ch == '\\' || ch == '/' {
                current_path.push(ch);
                break;
            }
        }
        let file_path = current_path + &String::from("help.qcpro");
        fs::File::create(file_path.clone())?;
        let contents = format!("\n{}\n",
"Example: qcpro [action] [subaction]
action:
  new    create new project name of subaction.
  init   initialize project for directory that name of subaction.
  cmake  use cmake to quick build projcet
subaction:
  <directory name>    the directory of project. If use action `init`, it can nothing and will initialize project on current directory.
                      If use `cmake`, it can run with two subactions one of source and other of build target.It can also be nothing and it will build with default path.
special:
  --help    Print help to screen.");
        
        fs::write(file_path, &contents)?;
        Ok(contents)
    }
}