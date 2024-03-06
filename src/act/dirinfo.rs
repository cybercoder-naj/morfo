use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub struct DirInfo {
    pub header_files: Vec<PathBuf>,
    pub c_files: Vec<PathBuf>,
}

pub fn get_dir_info(root: &Path) -> DirInfo {
    let mut header_files = Vec::new();
    let mut c_files = Vec::new();

    // Use walkdir to find all c and h files in subdirectories
    for entry in WalkDir::new(root) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(extension) = path.extension() {
                match extension.to_str() {
                    Some("h") => header_files.push(path.to_path_buf()),
                    Some("c") => c_files.push(path.to_path_buf()),
                    _ => (),
                }
            }
        }
    }

    DirInfo {
        header_files,
        c_files,
    }
}

// Write a test for get_dir_info
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn get_dir_info_same_dir() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let root = tmp_dir.path();

        // Create a few files
        let c_file = root.join("main.c");
        let other_file = root.join("main.rs");

        fs::write(&c_file, "").unwrap();
        fs::write(&other_file, "").unwrap();

        let dir_info = get_dir_info(root);
        assert_eq!(dir_info.header_files, Vec::<PathBuf>::new());
        assert_eq!(dir_info.c_files, vec![c_file]);
    }

    #[test]
    fn get_dir_info_subdir() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let root = tmp_dir.path();

        // Create a few files
        let c_file = root.join("main.c");
        fs::write(&c_file, "").unwrap();

        // Create a subdir
        let subdir = root.join("subdir");
        fs::create_dir(&subdir).unwrap();

        // Create a few files in the subdir
        let h_file_aux = subdir.join("aux.h");
        let c_file_aux = subdir.join("aux.c");

        fs::write(&h_file_aux, "").unwrap();
        fs::write(&c_file_aux, "").unwrap();

        let dir_info = get_dir_info(root);
        assert_eq!(dir_info.header_files, vec![h_file_aux]);
        assert_eq!(dir_info.c_files, vec![c_file, c_file_aux]);
    }
}
