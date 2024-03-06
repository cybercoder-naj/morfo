use std::{fs, path::PathBuf};

use regex::Regex;

use crate::error::MorfoResult;

#[allow(dead_code)]
pub fn get_all_includes(filepath: &PathBuf) -> MorfoResult<Vec<String>> {
    let mut includes = Vec::new();

    let contents = fs::read_to_string(filepath)?;
    let re = Regex::new(r#"#include\s*"(.*)""#).unwrap();

    for line in contents.lines() {
        if let Some(cap) = re.captures(line) {
            includes.push(cap[1].to_string());
        }
    }

    Ok(includes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_get_all_includes() {
        // Create a temporary file
        let tmp_dir = tempfile::tempdir().unwrap();
        let tmp_file = tmp_dir.path().join("main.c");
        fs::write(
            &tmp_file,
            r#"
            #include <stdio.h>
            #include "aux.h"
            #include <string.h>
            "#,
        )
        .unwrap();

        // create path to tmp_file
        let temp_file = tmp_file.to_str().unwrap();

        let includes = get_all_includes(&PathBuf::from(temp_file)).unwrap();
        assert_eq!(includes, vec!["aux.h"]);
    }
}
