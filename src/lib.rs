//! Quick c++ project manager, the full name of qcpro, writen by [rust](https://www.rust-lang.org/), is a simple application arm to create and initialize a c++ project.

pub mod project;
pub mod command;
pub mod config;
pub mod help;
pub mod addition;
pub use command::{Command};

#[cfg(test)]
mod test {
    
}
