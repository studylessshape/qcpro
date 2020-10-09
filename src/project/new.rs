use std::fs;
use std::io;

use super::initialize;

/// Create new directory and init project
pub fn new_project(directory: String) -> Result<String, io::Error> {
    match fs::create_dir(directory.clone()) {
        Ok(()) => initialize::init_project(directory.clone()),
        Err(e) => Err(e),
    }
}