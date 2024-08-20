use std::fmt::Display;

pub struct Mock {
    module: String,
}

impl Mock {
    pub fn new(module: &str) -> Self {
        Self {
            module: module.to_string(),
        }
    }

    pub fn with_in(&self, test_file: &str) -> bool {
        let mock = format!(r#"jest.mock("{}""#, self.module);
        test_file.contains(&mock)
    }
}

impl Display for Mock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"jest.mock("{}")"#, self.module)
    }
}
