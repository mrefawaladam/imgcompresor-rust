use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn collect_input_files(input: &Path) -> Result<Vec<PathBuf>> {
    let mut list = Vec::new();

    if input.is_file() {
        list.push(input.to_path_buf());
    } else if input.is_dir() {
        for entry in WalkDir::new(input).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();

            if p.is_file() {
                if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                    let ext = ext.to_lowercase();
                    if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "webp" | "tif" | "tiff" | "bmp") {
                        list.push(p.to_path_buf());
                    }
                }
            }
        }
    }

    Ok(list)
}

pub fn prepare_output_dir(output: &Path) -> Result<()> {
    use std::fs;

    if output.exists() {
        if output.is_file() {
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)?;
            }
        }
    } else {
        if output.extension().is_none() || 
           output.to_string_lossy().ends_with(std::path::MAIN_SEPARATOR) {
            fs::create_dir_all(output)?;
        } else {
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)?;
            }
        }
    }
    Ok(())
}
