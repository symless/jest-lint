use clap::Parser;
use colored::*;
use core::panic;
use regex::RegexBuilder;
use std::{
    fmt::Display,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

const DEFAULT_DIR_PATH: &str = ".";
const IGNORE_PATHS: [&str; 3] = ["node_modules", "build", "__snapshots__"];
const TEST_FILE_MATCH: &str = ".test";
const SPEC_FILE_MATCH: &str = ".spec";

struct TestPair {
    test_path: PathBuf,
    under_test_path: PathBuf,
}

impl Display for TestPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.test_path.display(),
            self.under_test_path.display()
        )
    }
}

#[derive(Parser)]
struct Args {
    /// Checks that all imports have a corresponding mock.
    #[arg(short, long)]
    mocks: bool,

    /// Only check a specific filename.
    #[arg(short, long)]
    filename: Option<String>,

    /// Directory to check. Defaults to '../..' if not set.
    #[arg(short, long)]
    directory: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.mocks {
        check_mocks(args.filename, args.directory);
        return;
    }

    println!("Oops, no command specified. Try --help.");
}

fn check_mocks(filename: Option<String>, start_dir_name: Option<String>) {
    if let Some(f_name) = filename {
        let pair = get_pair_for_single_file(f_name);

        if !pair.test_path.exists() {
            println!(
                "Sorry, test file doesn't exist: {}",
                pair.test_path.display()
            );
            return;
        }

        if !pair.under_test_path.exists() {
            println!(
                "Sorry, module under test doesn't exist: {}",
                pair.under_test_path.display()
            );
            return;
        }

        check_missing_mocks(vec![pair]);
        return;
    }

    let start_dir_name = start_dir_name.unwrap_or(DEFAULT_DIR_PATH.to_string());

    let start_dir = Path::new(&start_dir_name);
    let mut test_files = Vec::new();

    println!("Looking for files in: {}", start_dir.display());
    find_test_files(start_dir, &mut test_files);
    if test_files.is_empty() {
        println!("No files with '{TEST_FILE_MATCH}' in the name.");
        return;
    }
    print_test_files(&test_files);

    let pairs: Vec<TestPair> = find_under_test(&test_files);
    if pairs.is_empty() {
        println!("Couldn't find any modules under test.");
        return;
    }
    print_under_test(&pairs);

    println!("Checking that all imports have a mock.");
    check_missing_mocks(pairs);
}

fn check_missing_mocks(pairs: Vec<TestPair>) {
    for pair in pairs {
        println!("Checking {}... ", pair.under_test_path.display());
        let imports = get_imports_from_file(&pair.under_test_path);

        if !imports.is_empty() {
            print_imports(&imports);
            check_test_for_jest_mocks(&pair, &imports);
        } else {
            println!("  No imports.");
        }
    }
}

fn check_test_for_jest_mocks(pair: &TestPair, imports: &[String]) {
    let test_path = pair.test_path.clone();
    let test_contents = fs::read_to_string(test_path).unwrap();
    let mut missing_mocks = Vec::new();

    for import in imports {
        let mock = format!(r#"jest.mock("{import}")"#,);
        if !test_contents.contains(&mock) {
            missing_mocks.push(mock);
        }
    }

    if !missing_mocks.is_empty() {
        println!("{}", "  Missing mocks:".red());
        for mock in missing_mocks {
            println!("    {mock}");
        }
    } else {
        println!(
            "\n{} All your imports are mocked.\n",
            "Good job!".green().bold()
        );
    }
}

fn print_imports(imports: &[String]) {
    println!("  Imports:");
    for import in imports {
        println!("    {import}");
    }
}

fn get_imports_from_file(path: &Path) -> Vec<String> {
    if let Ok(contents) = fs::read_to_string(path) {
        let re = RegexBuilder::new(r#"import(?:.|\n)+"(.+)";"#)
            .swap_greed(true)
            .multi_line(true)
            .build()
            .unwrap();

        re.captures_iter(&contents)
            .flat_map(|capture| capture.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    } else {
        panic!("Error reading file.");
    }
}

fn print_under_test(pairs: &[TestPair]) {
    println!("Modules under test:");
    for pair in pairs {
        println!("{pair}");
    }
}

fn print_test_files(test_files: &[DirEntry]) {
    println!("Files with '{TEST_FILE_MATCH}' in the name:");
    for entry in test_files {
        println!("{entry:?}");
    }
}

fn get_pair_for_single_file(filename: String) -> TestPair {
    if filename.contains(SPEC_FILE_MATCH) {
        panic!("Filenames with '{SPEC_FILE_MATCH}' are not supported yet.",);
    }
    if !filename.contains(TEST_FILE_MATCH) {
        panic!("Only filenames with '{TEST_FILE_MATCH}' are supported.");
    }
    let test_path = PathBuf::from(filename);
    let under_test_path = get_under_test_path(&test_path);
    TestPair {
        test_path,
        under_test_path,
    }
}

fn get_under_test_path(test_path: &Path) -> PathBuf {
    let test_name = test_path.file_name().unwrap().to_str().unwrap_or("");
    let name = test_name.replace(TEST_FILE_MATCH, "");

    test_path.with_file_name(name)
}

fn find_test_files(path: &Path, test_files: &mut Vec<DirEntry>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            let name = entry_path.file_name().unwrap().to_str().unwrap_or("");
            if IGNORE_PATHS.contains(&name) {
                continue;
            }

            if entry_path.is_dir() {
                find_test_files(&entry_path, test_files);
            } else {
                let filename = entry.file_name();
                if let Some(filename_str) = filename.to_str() {
                    if filename_str.contains(TEST_FILE_MATCH) {
                        test_files.push(entry);
                    }
                }
            }
        }
    } else {
        panic!("Error reading directory contents.");
    }
}

fn find_under_test(test_files: &[DirEntry]) -> Vec<TestPair> {
    test_files
        .iter()
        .flat_map(|test_entry| {
            let test_entry_path = test_entry.path();
            let test_name = test_entry_path.file_name().unwrap().to_str().unwrap_or("");
            let name = test_name.replace(TEST_FILE_MATCH, "");
            let path = test_entry_path.with_file_name(name);
            if path.exists() {
                Some(TestPair {
                    test_path: test_entry_path.clone(),
                    under_test_path: path,
                })
            } else {
                None
            }
        })
        .collect()
}
