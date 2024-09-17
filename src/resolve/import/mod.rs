use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use relative::resolve_relative_path;
use tsconfig::{parse_tsconfig_file, resolve_tsconfig_alias, TsConfig};

use crate::utils::find_path::find_imports;

pub mod node_modules;
pub mod relative;
pub mod tsconfig;

#[derive(Debug)]
pub struct ImportResolver {
    tsconfig: Option<TsConfig>,
    extensions: Vec<&'static str>,
    visited: HashSet<PathBuf>,
    pub context: String,
}

impl ImportResolver {
    pub fn new() -> Self {
        Self {
            tsconfig: parse_tsconfig_file().unwrap_or_default(),
            extensions: vec!["ts", "tsx", "js", "jsx"],
            visited: HashSet::new(),
            context: String::new(),
        }
    }

    pub fn resolve_import(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        // 防止循环解析
        if self.visited.contains(path) {
            return Ok(());
        }
        self.visited.insert(path.to_path_buf());

        // 读取文件内容
        let content = fs::read_to_string(path)?;
        self.context.push_str(&format!("{}\n", &content));

        // 匹配 import 语句
        let import_paths = find_imports(&content);
        for import_path in import_paths {
            self.resolve_import_path(import_path, path)?;
        }

        Ok(())
    }

    pub fn resolve_import_path(
        &mut self,
        import: &str,
        current_file: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let current_dir = current_file.parent().ok_or("无法获取当前文件的父目录")?;

        let resolved_path = self.get_resolved_path(import, current_dir)?;

        if !resolved_path.is_empty() {
            for path in resolved_path {
                if self.visited.contains(&path) {
                    return Ok(());
                }
                self.visited.insert(path.clone());
                let content = fs::read_to_string(&path)?;
                self.context.push_str(&format!("{}\n", &content));
                self.resolve_import(&path)?;
            }
        }

        Ok(())
    }

    fn get_resolved_path(
        &self,
        import: &str,
        current_dir: &Path,
    ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut resolved_path = resolve_relative_path(import, current_dir, &self.extensions)?;

        if resolved_path.is_empty() {
            if let Some(compiler_options) =
                &self.tsconfig.as_ref().map(|t| t.compilerOptions.clone())
            {
                resolved_path = resolve_tsconfig_alias(import, compiler_options, &self.extensions)?;
            }
        }

        Ok(resolved_path)
    }
}
