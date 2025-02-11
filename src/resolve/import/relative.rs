use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::utils::try_search_target;

// 子函数：处理相对路径解析
pub fn resolve_relative_path(
    import: &str,
    current_dir: &Path,
    extensions: &[&str],
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if import.starts_with("./") || import.starts_with("../") {
        let relative_path = current_dir.join(import);
        return Ok(try_search_target(&relative_path, extensions));
    }
    Ok(vec![])
}
