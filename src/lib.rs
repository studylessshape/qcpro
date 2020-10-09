pub mod project;
pub mod command;
pub mod config;

pub use command::{Command, QcproReturnKind};
pub use config::help;

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::project::is_ignore_path;

    fn ignore() {
        let path=PathBuf::from("E:\\Project\\RustStudy\\qcpro\\test\\build");
        let ig = vec![String::from("build"), String::from("bin")];
        assert_eq!(is_ignore_path(&path, &ig), true);
    }

    #[test]
    fn test_regex() {
        std::process::Command::new("sh")
            .args(&["-c", "cd" ,"test/build", "make"])
            .spawn()
            .expect("Error");
        assert_eq!(2, 2);
    }
}
