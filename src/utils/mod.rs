pub mod find_path;

use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use find_path::find_exports;
use regex::Regex;

pub fn try_search_target(path: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if path.is_dir() {
        let index_path = path.join("index");
        if let Some(index_path) = try_find_with_extensions(&index_path, extensions) {
            result.push(index_path.clone());
            println!("Found target: {}", index_path.display());
        }
    }

    if let Some(path) = try_find_with_extensions(path, extensions) {
        result.push(path.clone());
        println!("Found target: {}", path.display());
    }

    let mut content = String::new();
    for path in &result {
        match fs::File::open(path) {
            Ok(mut file) => {
                file.read_to_string(&mut content).unwrap();
                break;
            }
            _ => continue,
        }
    }

    let exports = find_exports(&content);
    println!("Found exports: {:?}", exports);
    for export in exports {
        let mut export_result = try_search_target(&path.join(export), extensions);
        result.append(&mut export_result);
    }

    result
}


fn try_find_with_extensions(path: &Path, extensions: &[&str]) -> Option<PathBuf> {
    for ext in extensions {
        let path_with_extension = path.with_extension(ext);
        if path_with_extension.exists() {
            return Some(path_with_extension);
        }
    }
    None
}

pub fn remove_json_comments(json_content: &str) -> String {
    let re_multi_line = Regex::new(r"/\*.*?\*/").unwrap();
    let re_single_line = Regex::new(r"//.*\n").unwrap();
    let no_comment = json_content;

    let no_comment = re_multi_line.replace_all(no_comment, "");
    let no_comment = re_single_line.replace_all(&no_comment, "");

    no_comment.to_string()
}

pub fn get_absolute_path(base_url: Option<String>) -> String {
    let mut base_path = std::env::current_dir().unwrap();
    if let Some(url) = base_url {
        base_path.push(url);
    }
    base_path.display().to_string()
}
