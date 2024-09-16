use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;
use relative::resolve_relative_path;
use tsconfig::{parse_tsconfig_file, resolve_tsconfig_alias, TsConfig};

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
        let re = Regex::new(r#"import\s+.*?["'](.+?)["'];?"#)?;
        for caps in re.captures_iter(&content) {
            let import_path = &caps[1];
            let resolved_path = self.resolve_import_path(import_path, path)?;

            if let Some(resolved) = resolved_path {
                // 递归解析导入的文件
                self.resolve_import(&resolved)?;
            } else {
                // 如果解析失败，记录日志但继续
                eprintln!("无法解析导入路径: {}", import_path);
            }
        }

        Ok(())
    }

    pub fn resolve_import_path(
        &self,
        import: &str,
        current_file: &Path,
    ) -> Result<Option<PathBuf>, Box<dyn Error>> {
        let current_dir = current_file.parent().ok_or("无法获取当前文件的父目录")?;

        // 1. 处理相对路径
        if let Some(resolved_path) = resolve_relative_path(import, current_dir, &self.extensions)? {
            return Ok(Some(resolved_path));
        }

        // 2. 处理 tsconfig 别名
        if let Some(compiler_options) = &self.tsconfig.as_ref().map(|t| t.compilerOptions.clone())
        {
            if let Some(resolved_path) =
                resolve_tsconfig_alias(import, compiler_options, &self.extensions)?
            {
                return Ok(Some(resolved_path));
            }
        }

        // 3. 处理 node_modules 解析
        // if let Some(resolved_path) = resolve_node_modules(import, &self.extensions)? {
        //     return Ok(Some(resolved_path));
        // }

        Ok(None)
    }
}
