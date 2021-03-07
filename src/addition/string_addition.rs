use std::fs;

/// Get the first index by pattern
fn first_index(s: &String, pattern: &str) -> Option<usize> {
    let matched_pattern : Option<_> = s.match_indices(pattern).next();
    if let Some((index, _)) = matched_pattern {
        Some(index)
    } else {
        None
    }
}

/// Get the project name for file contents or last directory of path
pub fn get_project_name(source: &String, is_directory: bool) -> Option<String> {
    let mut s_cp = source.clone();
    let mut project_name = String::new();
    if is_directory{
        loop {
            if s_cp.len() < 1 {
                break;
            }
            let ch = s_cp.pop().unwrap();
            if ch == '\\' || ch == '/' {
                break;
            }
            project_name.insert(0, ch);
        }
    }else{
        if let Ok(contents) = fs::read(source) {
            let contents = String::from_utf8(contents).unwrap();
            let pat = "project(";
            let fir = match first_index(&contents, pat) {
                Some(n) => n,
                None => return None,
            };
            let contents: Vec<char> = contents.chars().collect();
            for idx in fir + pat.len()..contents.len() {
                if contents[idx] == ')' {
                    break;
                }
                project_name.push(contents[idx]);
            }
        }
    }

    if project_name.len() < 1 {
        return None;
    }else{
        return Some(project_name);
    }
}