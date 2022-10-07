use std::fs;
use std::fs::metadata;
use std::io::{Error, ErrorKind};
use indicatif::{ProgressBar, ProgressStyle};

pub const REQUIRED_PATHS: [&str; 2] = ["./assets/", "./output/"];

/// Returns files and directories in current directory
#[allow(dead_code)]
pub fn get_asset_names() -> Result<Vec<String>, Error> {
    let paths = fs::read_dir("./assets")?;

    // contains all directories under the "./assets" path
    let mut dirs = Vec::new();

    for path in paths {
        let path = path?;
        if metadata(path.path())?.is_dir() {
            dirs.push(path.file_name().into_string().unwrap());
        }
    }

    Ok(dirs)
}

/// Checks every paths in the REQUIRED_PATHS, it they not exists,
/// this function will create them
pub fn check_files() -> Result<(), Error> {
    for path in REQUIRED_PATHS {
        match fs::create_dir(path) {
            Err(err) => { if err.kind() != ErrorKind::AlreadyExists { return Err(err)}},
            Ok(_) => println!("{} created", path)
        }
    }

    Ok(())
}

/// We'll use the return value for randomization
pub fn get_asset_quantity(name: &str) -> Result<usize, Error> {
    let assets = fs::read_dir(REQUIRED_PATHS[0].to_owned() + name)?;

    Ok(assets.count())
}

/// Returns a progress bar
pub fn progress_bar(amount: usize) -> ProgressBar {
    let bar = ProgressBar::new(amount.try_into().unwrap());
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
            .unwrap()
            .progress_chars("##-"),
    );

    bar
}