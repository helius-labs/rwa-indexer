use std::path::PathBuf;

use git2::Repository;

pub fn get_relative_git_path(file_path: &str) -> PathBuf {
    let repo = Repository::discover(".").expect("Failed to discover Git repository");
    let git_root = repo.workdir().expect("Failed to get Git repository root");

    git_root.join(file_path)
}
