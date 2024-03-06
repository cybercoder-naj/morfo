use std::path::PathBuf;

use dirinfo::DirInfo;

mod builder;
pub mod dirinfo;

#[derive(Debug, PartialEq)]
pub struct ACT {
    pub name: String,
    pub header: Option<String>,
    pub linkers: Vec<String>,
    pub dependencies: Vec<ACT>,
}

impl ACT {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            header: Option::default(),
            linkers: Vec::default(),
            dependencies: Vec::default(),
        }
    }

    pub fn build(filepath: &PathBuf, dirinfo: &DirInfo) -> Self {
        let mut current = ACT::new(filepath.to_str().unwrap());

        let includes = builder::get_all_includes(filepath).unwrap();
        for include in includes {
            // find include in dirinfo.header_files
            for header in &dirinfo.header_files {
                if header.to_str().unwrap() != include {
                    continue;
                }

                // replace the .h with .c extension and find it in dirinfo.c_files
                let mut c_file = header.clone();
                c_file.set_extension("c");
                for c in &dirinfo.c_files {
                    if c.to_str().unwrap() != c_file.to_str().unwrap() {
                        continue;
                    }

                    // if found, add it as a dependency
                    let act = ACT::build(c, dirinfo);
                    current.dependencies.push(act);
                }
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act_new() {
        let act = ACT::new("main.c");
        assert_eq!(
            act,
            ACT {
                name: "main.c".to_string(),
                header: None,
                linkers: Vec::default(),
                dependencies: Vec::default(),
            }
        );
    }
}
