use std::io::Result;
use std::path::PathBuf;

pub fn get_project_root() -> Result<PathBuf> {
    if let Some(path) = get_cargo_workspace_root()? {
        Ok(path)
    } else {
        std::env::current_dir()
    }
}

pub fn get_cargo_workspace_root() -> Result<Option<PathBuf>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let current_path = PathBuf::from(manifest_dir).canonicalize()?;

    for ancestor in current_path.ancestors() {
        if ancestor.join("Cargo.lock").exists() {
            return Ok(Some(ancestor.to_path_buf()));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_root() {
        let project_root = get_project_root().unwrap();
        assert!(
            project_root.join("Cargo.toml").exists(),
            "Cargo.toml should exist in project root"
        );
    }

    #[test]
    fn test_get_workspace_root() {
        let workspace_root = get_cargo_workspace_root().unwrap();
        assert!(workspace_root.is_some(), "Workspace root should be Some");
        assert!(
            workspace_root.unwrap().join("Cargo.lock").exists(),
            "Cargo.lock should exist in workspace root"
        );
    }
}
