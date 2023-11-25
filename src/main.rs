use clap::{App, Arg};
use dirs;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    if env::var("DESTINATION").is_err() {
        println!("No destination directory set...");

        let dir = match dirs::home_dir() {
            Some(home_dir) => home_dir.join("Trash"),
            None => PathBuf::from("Trash"),
        };

        if !dir.exists() {
            fs::create_dir_all(&dir).expect("Failed to create destination directory");
        }

        env::set_var("DESTINATION", dir.to_str().unwrap());

        println!(
            "Succesfully set destination directory to {}",
            dir.to_str().unwrap()
        );
    }

    let matches = App::new("SafeRM")
        .version("1.0")
        .author("FireStreaker2")
        .about("Safely remove files via your terminal")
        .arg(
            Arg::with_name("source")
                .help("Sets the source directory")
                .required(true)
                .index(1),
        )
        .get_matches();

    let source_dir = matches.value_of("source").unwrap();
    let dest_dir = env::var("DESTINATION").unwrap_or_else(|_| ".".to_string());

    move_file(source_dir, &dest_dir);
}

fn move_file(source_file: &str, dest_dir: &str) {
    let source_path = Path::new(source_file);
    let dest_path = Path::new(dest_dir);

    if source_path.exists() {
        let file_name = source_path.file_name().unwrap();
        let dest_file_path = dest_path.join(file_name);

        fs::rename(source_path, dest_file_path).expect("Failed to move file");

        println!("File moved successfully!");
    } else {
        eprintln!("Source file does not exist.");
    }
}
