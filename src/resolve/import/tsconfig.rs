#![allow(non_snake_case)]

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};

use crate::utils::{get_absolute_path, remove_json_comments, try_search_target};

#[derive(Deserialize, Serialize, Debug)]
pub struct TsConfig {
    pub compilerOptions: CompilerOptions,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CompilerOptions {
    baseUrl: String,
    paths: HashMap<String, Vec<String>>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            baseUrl: "./".to_string(),
            paths: HashMap::new(),
        }
    }
}

pub struct Alias {
    from: String,
    to: String,
}

pub fn parse_tsconfig_file() -> Result<Option<TsConfig>, Box<dyn Error>> {
    let tsconfig_path = Path::new("tsconfig.json");
    if tsconfig_path.exists() {
        let tsconfig_content = fs::read_to_string(tsconfig_path)?;
        let tsconfig_content = remove_json_comments(&tsconfig_content);

        let tsconfig_json: TsConfig = serde_json::from_str(&tsconfig_content)?;
        return Ok(Some(tsconfig_json));
    }
    Ok(None)
}

// 子函数：处理 tsconfig 别名解析
pub fn resolve_tsconfig_alias(
    import: &str,
    compiler_options: &CompilerOptions,
    extensions: &[&str],
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let base_url = &compiler_options.baseUrl;
    let paths = &compiler_options.paths;

    for (pattern, replacements) in paths {
        let replacement = replacements.first().unwrap();
        let resolved = convert_import_statement(
            import,
            Alias {
                from: pattern.to_string(),
                to: replacement.to_string(),
            },
            base_url,
        );
        if let Some(found_path) = try_search_target(Path::new(&resolved), extensions) {
            return Ok(Some(found_path));
        }
    }
    Ok(None)
}

fn convert_import_statement(import_statement: &str, alias: Alias, base_url: &str) -> String {
    // 将 from 中的通配符 * 转换为正则表达式的 (.+)，表示匹配任意字符
    let from_escaped = regex::escape(&alias.from).replace(r"\*", "(.+)");
    let to_escaped = format!(
        "{}{}",
        get_absolute_path(Some(base_url.to_string())),
        alias.to
    )
    .replace('*', "$1"); // 将 * 替换为捕获组 $1


    // 创建正则表达式
    let re = Regex::new(&from_escaped).unwrap();

    // 替换 import_statement 中匹配的路径
    re.replace_all(import_statement, to_escaped).to_string()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_convert_import_statement() {
//         let import_statement = "import { foo } from '@/a/b/c'";

//         let result = convert_import_statement(
//             import_statement,
//             Alias {
//                 from: "@/*".to_string(),
//                 to: "./src/*".to_string(),
//             },
//             "./",
//         );
//         assert_eq!(result, "import { foo } from './src/a/b/c'");
//     }

//     #[test]
//     fn test_convert_import_statement_with_base_url() {
//         let import_statement = "import { foo } from '@/a/b/c'";
//         let base_url = "./src";
//         let result = convert_import_statement(
//             import_statement,
//             Alias {
//                 from: "@/*".to_string(),
//                 to: "./src/*".to_string(),
//             },
//             base_url,
//         );
//         assert_eq!(result, "import { foo } from './src/src/a/b/c'");
//     }
// }
