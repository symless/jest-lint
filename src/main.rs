use clap::Parser;
use colored::*;
use module::Module;
use regex::Regex;
use std::{
    fs::{self},
    path::{Path, PathBuf},
    sync::LazyLock,
};
use test_pair::{find_all_tests_in_directory, TestPair};

mod module;
mod test_pair;

const DEFAULT_DIR_PATH: &str = ".";

static IMPORT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"import\s+(\{[^}]+\}|\*\s+as\s\w+|\w+)\s+from\s+"([^"]+)"#).unwrap()
});
static IGNORE_IMPORT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)//#region not-mocked.*?//#endregion").unwrap());

#[derive(Parser)]
struct Args {
    /// Checks that all imports have a corresponding mock.
    #[arg(short, long)]
    mocks: bool,

    /// Only check a specific filename.
    #[arg(short, long)]
    filename: Option<PathBuf>,

    /// Directory to check. Defaults to '.' if not set.
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if args.mocks {
        check_mocks(args.filename, args.directory);
        return;
    }

    println!("Oops, no command specified. Try --help.");
}

fn check_mocks(filename: Option<PathBuf>, start_dir_name: Option<PathBuf>) {
    if let Some(f_name) = filename {
        match TestPair::try_from(f_name) {
            Ok(pair) => check_missing_mocks(vec![pair]),
            Err(err) => eprintln!("{err}"),
        };
        return;
    }

    let start_dir_name = start_dir_name.unwrap_or(PathBuf::from(DEFAULT_DIR_PATH));

    let start_dir = Path::new(&start_dir_name);

    println!("Looking for files in: {}", start_dir.display());
    let pairs = find_all_tests_in_directory(start_dir);
    if pairs.is_empty() {
        println!("Could not find test file pairs");
        return;
    }

    print_under_test(&pairs);

    println!("Checking that all imports have a mock.");
    check_missing_mocks(pairs);
}

fn check_missing_mocks(pairs: Vec<TestPair>) {
    for pair in pairs {
        println!("Checking {}... ", pair.under_test_file.display());
        let imports = get_imports_from_file(&pair.under_test_file);

        if !imports.is_empty() {
            print_imports(&imports);
            check_test_for_jest_mocks(&pair, &imports);
        } else {
            println!("  No imports.");
        }
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
