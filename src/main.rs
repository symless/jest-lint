use crate::{
    cli::Args,
    config::{find_config, Config},
    module::Module,
    test_pair::{find_all_tests_in_directory, find_test_pairs_for_files, TestPair},
};
use clap::Parser;
use colored::*;
use core::slice;
use regex::Regex;
use std::{fs, path::{Path, PathBuf}, process, sync::LazyLock};

mod cli;
mod config;
mod module;
mod test_pair;

static IMPORT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"import\s+(\{[^}]+\}|\*\s+as\s\w+|\w+)\s+from\s+"([^"]+)"#).unwrap()
});
static IGNORE_IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)//#region not-mocked.*?//#endregion").unwrap());
static IGNORE_COMMENT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//\s*jest_lint:ignore\s+(.+)").unwrap());
static JEST_MOCK_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"jest\.mock\("([^"]+)""#).unwrap());

fn main() {
    let args = Args::parse();

    let has_errors = match (args.mocks, args.filename) {
        (true, Some(path)) => {
            let config = find_config(&path);
            check_file_mock(&path, &config)
        }
        (true, None) if !args.files.is_empty() => {
            let config = find_config(&args.directory);
            check_files_mocks(&args.files, &config)
        }
        (true, None) => {
            let config = find_config(&args.directory);
            check_directory_mocks(&args.directory, &config)
        }
        _ => {
            println!("Oops, no command specified. Try --help.");
            false
        }
    };

    if has_errors {
        process::exit(1);
    }
}

fn check_file_mock(path: &Path, config: &Config) -> bool {
    match TestPair::try_from(path) {
        Ok(pair) => check_missing_mocks(slice::from_ref(&pair), config),
        Err(err) => {
            eprintln!("{err}");
            true
        }
    }
}

fn check_files_mocks(files: &[PathBuf], config: &Config) -> bool {
    let pairs = find_test_pairs_for_files(files);
    if pairs.is_empty() {
        println!("No test file pairs found for the given files.");
        return false;
    }

    print_under_test(&pairs);
    println!("Checking that all imports have a mock.");
    check_missing_mocks(&pairs, config)
}

fn check_directory_mocks(path: &Path, config: &Config) -> bool {
    println!("Looking for files in: {}", path.display());
    let pairs = find_all_tests_in_directory(path);
    if pairs.is_empty() {
        println!("Could not find test file pairs");
        return false;
    }

    print_under_test(&pairs);
    println!("Checking that all imports have a mock.");
    check_missing_mocks(&pairs, config)
}

fn check_missing_mocks(pairs: &[TestPair], config: &Config) -> bool {
    let mut has_errors = false;
    for pair in pairs {
        println!("Checking {}... ", pair.module_file.display());
        let all_imports = get_all_imports_from_file(&pair.module_file);

        if all_imports.is_empty() {
            println!("  No imports.");
            continue;
        }

        if check_test_for_jest_mocks(pair, &all_imports, config) {
            has_errors = true;
        }
    }
    has_errors
}

fn check_test_for_jest_mocks(
    pair: &TestPair,
    all_imports: &[Module],
    config: &Config,
) -> bool {
    let test_contents = fs::read_to_string(&pair.test_file).unwrap();
    let test_ignores = get_test_ignores(&test_contents);
    let test_mocks = get_test_mocks(&test_contents);

    let mut missing_mocks = Vec::new();

    println!("  Imports:");
    for module in all_imports {
        if config.is_ignored(module.name()) {
            println!("    {} {}", module, "(ignored)".dimmed());
        } else if module.mock_with_in(&test_contents) {
            println!("    {} {}", module, "(mocked)".green());
        } else if module.in_list(&test_ignores) {
            println!("    {} {}", module, "(ignored)".dimmed());
        } else {
            println!("    {} {}", module, "(not mocked)".red());
            missing_mocks.push(module);
        }
    }

    let has_missing = !missing_mocks.is_empty();
    if has_missing {
        println!("{}", "  Missing mocks:".red());
        for module in &missing_mocks {
            println!("    {}", module.mock());
        }
    }

    let warnings = get_warnings(&test_mocks, all_imports, config);
    let has_warnings = !warnings.is_empty();
    if has_warnings {
        println!("{}", "  Warnings:".yellow());
        for warning in &warnings {
            println!("    {warning}");
        }
    }

    if !has_missing && !has_warnings {
        println!(
            "\n{} All your imports are mocked.\n",
            "Good job!".green().bold()
        );
    }

    has_missing
}

fn get_warnings(test_mocks: &[String], all_imports: &[Module], config: &Config) -> Vec<String> {
    let mut warnings = Vec::new();

    for mock in test_mocks {
        if config.is_ignored(mock) {
            warnings.push(format!(
                "jest.mock(\"{mock}\") is unnecessary (module is globally ignored)"
            ));
        } else if !all_imports.iter().any(|import| import.name() == mock) {
            warnings.push(format!(
                "jest.mock(\"{mock}\") does not match any import in the module under test"
            ));
        }
    }

    warnings
}

fn get_test_mocks(test_contents: &str) -> Vec<String> {
    JEST_MOCK_REGEX
        .captures_iter(test_contents)
        .filter_map(|capture| capture.get(1))
        .map(|m| m.as_str().to_string())
        .collect()
}

fn get_test_ignores(test_contents: &str) -> Vec<String> {
    IGNORE_COMMENT_REGEX
        .captures_iter(test_contents)
        .filter_map(|capture| capture.get(1))
        .flat_map(|m| m.as_str().split(',').map(|s| s.trim().to_string()))
        .collect()
}

fn get_all_imports_from_file(path: &Path) -> Vec<Module> {
    let contents = fs::read_to_string(path).expect("Error reading file.");
    let filtered_contents = IGNORE_IMPORT_REGEX.replace_all(&contents, "");
    IMPORT_REGEX
        .captures_iter(&filtered_contents)
        .filter_map(|capture| capture.get(2))
        .map(|m| Module::new(m.as_str()))
        .collect()
}

fn print_under_test(pairs: &[TestPair]) {
    println!("Modules under test:");
    for pair in pairs {
        println!("{pair}");
    }
}
