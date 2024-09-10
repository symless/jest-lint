use crate::{
    cli::Args,
    module::Module,
    test_pair::{find_all_tests_in_directory, TestPair},
};
use clap::Parser;
use colored::*;
use core::slice;
use regex::Regex;
use std::{fs, path::Path, sync::LazyLock};

mod cli;
mod module;
mod test_pair;

static IMPORT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"import\s+(\{[^}]+\}|\*\s+as\s\w+|\w+)\s+from\s+"([^"]+)"#).unwrap()
});
static IGNORE_IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"//#region not-mocked.*?//#endregion").unwrap());

fn main() {
    let args = Args::parse();

    match (args.mocks, args.filename) {
        (true, Some(path)) => check_file_mock(&path),
        (true, None) => check_directory_mocks(&args.directory),
        _ => println!("Oops, no command specified. Try --help."),
    }
}

fn check_file_mock(path: &Path) {
    match TestPair::try_from(path) {
        Ok(pair) => check_missing_mocks(slice::from_ref(&pair)),
        Err(err) => eprintln!("{err}"),
    };
}

fn check_directory_mocks(path: &Path) {
    println!("Looking for files in: {}", path.display());
    let pairs = find_all_tests_in_directory(path);
    if pairs.is_empty() {
        println!("Could not find test file pairs");
        return;
    }

    print_under_test(&pairs);
    println!("Checking that all imports have a mock.");
    check_missing_mocks(&pairs);
}

fn check_missing_mocks(pairs: &[TestPair]) {
    for pair in pairs {
        println!("Checking {}... ", pair.module_file.display());
        let imports = get_imports_from_file(&pair.module_file);

        if imports.is_empty() {
            println!("  No imports.");
            continue;
        }

        print_imports(&imports);
        check_test_for_jest_mocks(pair, &imports);
    }
}

fn check_test_for_jest_mocks(pair: &TestPair, modules: &[Module]) {
    let test_contents = fs::read_to_string(&pair.test_file).unwrap();
    let missing_mocks: Vec<&Module> = modules
        .iter()
        .filter(|module| !module.mock_with_in(&test_contents))
        .collect();

    if missing_mocks.is_empty() {
        println!(
            "\n{} All your imports are mocked.\n",
            "Good job!".green().bold()
        );
    } else {
        println!("{}", "  Missing mocks:".red());
        for module in missing_mocks {
            println!("    {}", module.mock());
        }
    }
}

fn print_imports(modules: &[Module]) {
    println!("  Imports:");
    for module in modules {
        println!("    {module}");
    }
}

fn get_imports_from_file(path: &Path) -> Vec<Module> {
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
