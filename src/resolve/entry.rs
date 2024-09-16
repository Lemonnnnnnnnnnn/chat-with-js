use std::error::Error;
use std::path::Path;

use super::import::ImportResolver;

pub fn resolve_entry(entry_path: &Path) -> Result<String, Box<dyn Error>> {
    let mut resolver = ImportResolver::new();
    resolver.resolve_import(entry_path)?;

    Ok(resolver.context)
}
