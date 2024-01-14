pub fn file_name(path: &str) -> String {
    let file_name = path.split("/").last().unwrap().to_string();
    file_name.split(".").next().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act_file_name() {
        assert_eq!(file_name("main.c"), "main");

        assert_eq!(file_name("src/main.cpp"), "main");
    }
}
