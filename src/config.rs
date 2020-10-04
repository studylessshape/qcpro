/// This mod is about the config of program
/// It's contains:
///     - Generate config file
///     - Read config file
///     - Init program

pub mod help{
    use std::io;
    use std::fs;

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
        let file_name = "help.qcpro";
        fs::File::create(file_name)?;
        let contents = format!("\n{}\n{}\n{}\n{}\n{}\n{}\n",
                        "Example: qcpro [action] [subaction]",
                        "  action:",
                        "    new    create new project name of subaction",
                        "    init   initialize project for directory that name of subaction",
                        "  subaction:",
                        "    <directory name>    the directory of project. If use action `init`, it can nothing and will initialize project on current directory");
        
        fs::write(file_name, &contents)?;
        Ok(contents)
    }
}