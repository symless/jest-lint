use clap::Parser;
use std::{env, ffi::OsString, path::PathBuf};

fn get_current_directory() -> OsString {
    env::current_dir().unwrap().into_os_string()
}

#[derive(Debug, Parser)]
pub struct Args {
    /// Checks that all imports have a corresponding mock.
    #[arg(short, long)]
    pub mocks: bool,

    /// Only check a specific filename.
    #[arg(short, long)]
    pub filename: Option<PathBuf>,

    /// Directory to check. Defaults to '.' if not set.
    #[arg(short, long)]
    #[arg(default_value = get_current_directory().to_os_string())]
    pub directory: PathBuf,
}
