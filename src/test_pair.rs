use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

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
    pub module_file: PathBuf,
}

impl TestPair {
    fn new(test_file: impl Into<PathBuf>, under_test_file: impl Into<PathBuf>) -> Self {
        Self {
            test_file: test_file.into(),
            module_file: under_test_file.into(),
        }
    }
}

impl Display for TestPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.test_file.display(),
            self.module_file.display()
        )
    }
}

impl TryFrom<&Path> for TestPair {
    type Error = Error;

    fn try_from(test_file: &Path) -> Result<Self, Self::Error> {
        if !test_file.exists() {
            return Err(Error::TestFileDoesNotExist(test_file.into()));
        }

        let test_name = test_file.file_name().unwrap().to_str().unwrap_or("");
        if !test_name.contains(TEST_FILE_EXT) && !test_name.contains(SPEC_FILE_EXT) {
            return Err(Error::NotTestFile(test_file.into()));
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

impl TryFrom<DirEntry> for TestPair {
    type Error = Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        TestPair::try_from(value.path())
    }
}

pub fn find_all_tests_in_directory(path: impl AsRef<Path>) -> Vec<TestPair> {
    WalkDir::new(path.as_ref())
        .into_iter()
        .filter_entry(|e| {
            let entry_file = e.path();
            let name = entry_file
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("");
            !IGNORE_PATHS.contains(&name)
        })
        .flatten()
        .flat_map(TestPair::try_from)
        .collect()
}
