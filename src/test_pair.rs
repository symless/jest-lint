use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

const TEST_FILE_EXT: &str = ".test";
const SPEC_FILE_EXT: &str = ".spec";
const IGNORE_PATHS: [&str; 3] = ["node_modules", "build", "__snapshots__"];

#[derive(Error, Debug)]
pub enum Error {
    #[error("Sorry, test file doesn't exist: {0}")]
    TestFileDoesNotExist(PathBuf),
    #[error("'{0}' is not a test file")]
    NotTestFile(PathBuf),
    #[error("Sorry, module under test doesn't exist: {0}")]
    UnderTestFileDoesNotExist(PathBuf),
}

pub struct TestPair {
    pub test_file: PathBuf,
    pub under_test_file: PathBuf,
}

impl TestPair {
    fn new(test_file: impl Into<PathBuf>, under_test_file: impl Into<PathBuf>) -> Self {
        Self {
            test_file: test_file.into(),
            under_test_file: under_test_file.into(),
        }
    }
}

impl Display for TestPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.test_file.display(),
            self.under_test_file.display()
        )
    }
}

impl TryFrom<PathBuf> for TestPair {
    type Error = Error;

    fn try_from(test_file: PathBuf) -> Result<Self, Self::Error> {
        if !test_file.exists() {
            return Err(Error::TestFileDoesNotExist(test_file));
        }

        let test_name = test_file.file_name().unwrap().to_str().unwrap_or("");
        if !test_name.contains(TEST_FILE_EXT) && !test_name.contains(SPEC_FILE_EXT) {
            return Err(Error::NotTestFile(test_file));
        }

        let name = test_name
            .replace(TEST_FILE_EXT, "")
            .replace(SPEC_FILE_EXT, "");
        let under_test_file = test_file.with_file_name(name);
        if !(under_test_file.exists()) {
            return Err(Error::UnderTestFileDoesNotExist(under_test_file));
        }
        Ok(Self::new(test_file, under_test_file))
    }
}

fn find_all_tests_in_directory_internal(path: &Path, tests: &mut Vec<TestPair>) {
    let entries = fs::read_dir(path).expect("Error reading directory contents.");
    for entry in entries.flatten() {
        let entry_file = entry.path();
        let name = entry_file.file_name().unwrap().to_str().unwrap_or("");
        if IGNORE_PATHS.contains(&name) {
            continue;
        }

        if entry_file.is_dir() {
            find_all_tests_in_directory_internal(&entry_file, tests);
        } else if let Ok(test_pair) = TestPair::try_from(entry.path()) {
            tests.push(test_pair);
        }
    }
}

pub fn find_all_tests_in_directory(path: impl AsRef<Path>) -> Vec<TestPair> {
    let mut tests: Vec<TestPair> = vec![];
    find_all_tests_in_directory_internal(path.as_ref(), &mut tests);
    tests
}
