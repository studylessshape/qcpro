
use std::io;

pub fn print_version() {
    println!("qcpro --version 0.2 stable");
}

/// Through reading file or creating file, get the string of help.Then print on cmd or shell
pub fn print_help() -> Result<(), io::Error> {
    println!("{}\n",
    "Example: qcpro [action] [subaction]
    action:
      new      create new project name of subaction
      init     initialize project for directory that name of subaction
      build    use cmake to quickly build projcet
      run      windows: use g++ to simply and quickly compile project
               shell: use cmake to build and use make to compile project
    subaction:
      <directory name>    the directory of project. If use action `init`, it can nothing and will initialize project on current directory
                          If use `cmake`, it can run with two subactions one of source and other of build target.It can also be nothing and it will build with default path
    option:
      -h,--help      Print help to screen
      -v, --version  qcpro version");
    Ok(())
}
