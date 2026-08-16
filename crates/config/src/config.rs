use std::{fs, path::{Path, PathBuf}};

fn find_service_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            result.extend(find_service_files(&path)?);
        } else if path.extension().and_then(|e| e.to_str()) == Some("service") {
            result.push(path);
        }
    }

    Ok(result)
}