use colored::*;
use std::fs;
use std::io::*;

fn list_files_in_path(path: &std::path::Path) {
    match fs::read_dir(path) {
        Ok(dir) => {
            println!("\nFound files:");

            for entry in dir {
                let entry = entry.expect("Unable to read entry");
                let path = entry.path();
                println!(
                    "→ {:?}",
                    path.file_name().unwrap().to_str().unwrap().to_string()
                );
            }
        }
        Err(_) => {
            println!("{}", "Unable to read directory".red());
        }
    };
}

fn main() {
    let mut path = std::env::current_dir().unwrap();
    let mut locales_path = String::new();

    println!("Enter the path to your locales folder: ");

    print!("{}", (path.to_str().unwrap().to_string() + "/").black());
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut locales_path)
        .expect("Could not read line");
    path = path.join(locales_path.trim());

    // println!("Listing files in {:?}", path);

    list_files_in_path(&path);
}
