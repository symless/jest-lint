use std::fmt::Display;

#[derive(Debug)]
pub struct Module(String);

impl Module {
    pub fn new(module: &str) -> Self {
        Self(module.to_string())
    }

    pub fn mock_with_in(&self, test_file: &str) -> bool {
        let mock = format!(r#"jest.mock("{}""#, self.0);
        test_file.contains(&mock)
    }

    pub fn mock(&self) -> String {
        format!(r#"jest.mock("{}")"#, self.0)
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
