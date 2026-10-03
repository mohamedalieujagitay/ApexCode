use git2::{Repository, StatusOptions, StatusShow};
use anyhow::Result;

fn get_staged_files() -> Result<Vec<String>> {
    let repo = Repository::open(".")?;
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    let mut files = Vec::new();
    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            if entry.status().is_index_new() || entry.status().is_index_modified() {
                files.push(path.to_string());
            }
        }
    }

    Ok(files)
}

fn get_all_files_in_dir() -> Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }
    Ok(files)
}