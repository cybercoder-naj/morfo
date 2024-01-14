mod builder;

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

    pub fn build(name: &str) -> Self {
        let main = ACT::new(name);

        main
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act_new() {
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
