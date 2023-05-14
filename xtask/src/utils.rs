use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use xshell::Shell;

pub(crate) fn find_files<P: AsRef<Path>>(dir: P, extension: &str) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    let dir_path = dir.as_ref();
    find_files_recursive(dir_path, extension, &mut result)?;

    let relative_paths: Vec<PathBuf> = result
        .into_iter()
        .filter_map(|path| path.strip_prefix(dir_path).ok().map(PathBuf::from))
        .collect();

    Ok(relative_paths)
}

fn find_files_recursive(dir: &Path, extension: &str, result: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            find_files_recursive(&path, extension, result)?;
        } else if path.is_file() && path.extension().map_or(false, |ext| ext == extension) {
            result.push(path);
        }
    }
    Ok(())
}

pub(crate) fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Failed to find project root")
        .to_path_buf()
}

pub(crate) fn verbose_cd<P: AsRef<Path>>(sh: &Shell, dir: P) {
    sh.change_dir(dir);
    eprintln!(
        "\n$ cd {}{}",
        sh.current_dir().display(),
        std::path::MAIN_SEPARATOR
    );
}
