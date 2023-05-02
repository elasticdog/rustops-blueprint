use std::{
    fs,
    path::{Path, PathBuf},
};

use xshell::{cmd, Shell};

use crate::{project_root, verbose_cd, DynError};

pub fn format_cue() -> Result<(), DynError> {
    let sh = Shell::new()?;
    verbose_cd(&sh, cue_dir());
    cmd!(sh, "cue fmt --simplify").run()?;
    Ok(())
}

pub fn format_markdown() -> Result<(), DynError> {
    let sh = Shell::new()?;
    let root = project_root();
    verbose_cd(&sh, &root);

    let markdown_files = find_markdown_files(sh.current_dir())?;
    let relative_paths: Vec<PathBuf> = markdown_files
        .into_iter()
        .filter_map(|path| path.strip_prefix(&root).ok().map(PathBuf::from))
        .collect();

    cmd!(sh, "prettier --prose-wrap always --write")
        .args(&relative_paths)
        .run()?;

    Ok(())
}

pub fn format_rust() -> Result<(), DynError> {
    let sh = Shell::new()?;
    verbose_cd(&sh, project_root());
    cmd!(sh, "cargo fmt").run()?;
    Ok(())
}

pub fn lint_rust() -> Result<(), DynError> {
    let sh = Shell::new()?;
    verbose_cd(&sh, project_root());
    cmd!(sh, "cargo clippy --fix --allow-no-vcs -- -D warnings").run()?;
    Ok(())
}

pub fn regenerate_ci_yaml() -> Result<(), DynError> {
    let sh = Shell::new()?;
    verbose_cd(&sh, cue_dir());
    cmd!(sh, "cue cmd regen-ci-yaml").run()?;
    Ok(())
}

pub fn spellcheck() -> Result<(), DynError> {
    let sh = Shell::new()?;
    verbose_cd(&sh, project_root());
    cmd!(sh, "codespell --write-changes").run()?;
    Ok(())
}

fn cue_dir() -> PathBuf {
    project_root().join(".github/cue")
}

fn find_markdown_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>, DynError> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            let mut subdir_files = find_markdown_files(&path)?;
            result.append(&mut subdir_files);
        } else if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            result.push(path);
        }
    }
    Ok(result)
}