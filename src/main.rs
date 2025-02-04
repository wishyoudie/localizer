use colored::*;
use serde_json::{json, Value};
use std::fs;
use std::io::*;

fn traverse_directory(path: &std::path::Path, cb: &dyn Fn(&std::path::Path)) {
    match fs::read_dir(path) {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.expect("Unable to read entry");
                cb(&entry.path());
            }
        }
        Err(_) => {
            eprintln!("{}", "Unable to read directory".red());
            std::process::exit(1);
        }
    };
}

fn load_from_stdin(var: &mut String, name: Option<&str>, prefix: Option<&String>) {
    if let Some(name_str) = name {
        println!("Enter {}: ", name_str);
    }

    if let Some(prefix_str) = prefix {
        print!("{}", prefix_str.black());
        stdout().flush().unwrap();
    }

    stdin().read_line(var).expect("Could not read line");
}

fn set_json_value(file_path: &str, key_path: &str, new_value: &str) -> Result<()> {
    // Read and parse the JSON file
    let content = fs::read_to_string(file_path)?;
    let mut json: Value = serde_json::from_str(&content)?;

    // Split the path into parts
    let path_parts: Vec<&str> = key_path.split('.').collect();

    // Navigate through the JSON structure
    let mut current = &mut json;
    for (i, part) in path_parts.iter().enumerate() {
        if i == path_parts.len() - 1 {
            // We're at the final part - check if value exists
            if current.get(part).is_some() {
                println!(
                    "{}",
                    format!("Warning: Overwriting existing value at {}", key_path).yellow()
                );
            }
            // Set the new value
            *current
                .as_object_mut()
                .expect("JSON structure is invalid")
                .entry(part.to_string())
                .or_insert(Value::Null) = Value::String(new_value.to_string());
        } else {
            // Create nested objects if they don't exist
            if !current.get(part).is_some() {
                current
                    .as_object_mut()
                    .expect("JSON structure is invalid")
                    .insert(part.to_string(), json!({}));
            }
            current = current
                .get_mut(part)
                .expect("Failed to navigate JSON structure");
        }
    }

    // Write back to file
    let formatted = serde_json::to_string_pretty(&json)?;
    fs::write(file_path, formatted)?;
    Ok(())
}

fn create_file_handler(key: &String, value: &String) -> impl Fn(&std::path::Path) {
    let key = key.clone().trim().to_string();
    let value = value.clone().trim().to_string();

    move |path: &std::path::Path| match set_json_value(path.to_str().unwrap(), &key, &value) {
        Ok(_) => {
            println!(
                "{}",
                format!(
                    " • Successfully updated {} in {}",
                    key,
                    path.file_name().unwrap().to_str().unwrap().to_string()
                )
                .green()
            );
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: app [OPTIONS]");
    println!("Options:");
    println!("  --path    Relative path to locales folder");
    println!("  --key     JSON key to update (e.g., 'common.greeting')");
    println!("  --value   Value to set for the key");
    println!("  --help    Print this help message");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut path = std::env::current_dir().unwrap();
    let mut locales_path = String::new();
    let mut key = String::new();
    let mut value = String::new();
    let mut i = 1;

    // Parse command line arguments
    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                print_usage();
                return;
            }
            "--path" => {
                if i + 1 < args.len() {
                    locales_path = args[i + 1].clone();
                    i += 2;
                    continue;
                }
            }
            "--key" => {
                if i + 1 < args.len() {
                    key = args[i + 1].clone();
                    i += 2;
                    continue;
                }
            }
            "--value" => {
                if i + 1 < args.len() {
                    value = args[i + 1].clone();
                    i += 2;
                    continue;
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if !locales_path.is_empty() {
        path = path.join(locales_path.trim());
        println!("Using path: {}", path.display());
    } else {
        load_from_stdin(
            &mut locales_path,
            Some("path to your locales folder"),
            Some(&(path.to_str().unwrap().to_string() + "/")),
        );
        path = path.join(locales_path.trim());
    }

    if key.is_empty() {
        load_from_stdin(&mut key, None, Some(&("JSON Key: ".to_string())));
    }

    if value.is_empty() {
        load_from_stdin(&mut value, None, Some(&("Value: ".to_string())));
    }

    let file_handler = create_file_handler(&key, &value);
    traverse_directory(&path, &file_handler);
}
