extern crate colored;
extern crate toml;

use std::{env, fs, process::Command};

use colored::*;

fn get_version(crate_name: &str) -> Result<String, String> {
    if let Ok(resp) = Command::new("cargo").arg("search").arg("--registry").arg("crates-io").arg(crate_name).output() {
        let resp_string = String::from_utf8_lossy(&resp.stdout)
            .to_owned()
            .split("\n...")
            .next()
            .unwrap()
            .to_string();
        if resp_string.is_empty() {
            return Err(format!("crates.io has no crate named {:?}", crate_name));
        }
        if let Ok(value) = toml::from_str::<toml::Value>(&resp_string) {
            let version = value.as_table().unwrap()[crate_name]
                .as_str()
                .unwrap()
                .to_string();
            Ok(version)
        } else {
            Err("cargo search returned invalid data".to_string())
        }
    } else {
        Err("Error running cargo search".to_string())
    }
}

static USAGE_STRING: &'static str = "\
Usage:
    cargo stabilize [flags]

Flags:
    -h | --help     Display usage information
    --upgrade       Upgrade all dependency versions to the newest,
                    not just wilcards
";

fn main() {
    // Read command-line arguments
    let mut upgrade = false;
    for arg in env::args().skip_while(|s| s != "stabilize").skip(1) {
        match arg.as_ref() {
            "-h" | "--help" => println!("{}", USAGE_STRING),
            "--upgrade" => upgrade = true,
            _ => println!("Unknown command\n{}", USAGE_STRING),
        }
    }

    // Read in the Cargo.toml
    match fs::read("Cargo.toml") {
        // Parse into toml::Value
        Ok(cargo_toml) => match toml::from_slice::<toml::Value>(&cargo_toml) {
            Ok(mut cargo_toml_value) => {
                // Ensure that the toml::Value is a table
                if let toml::Value::Table(ref mut map) = cargo_toml_value {
                    // Look for the dependencies
                    if let Some(dependencies) = map.get_mut("dependencies") {
                        // Ensure that the dependencies are a table
                        if let toml::Value::Table(ref mut map) = dependencies {
                            let mut stabilized = 0;
                            let mut upgraded = 0;
                            // Iterate through each dependency
                            for (crate_name, dep_value) in map {
                                // Get the version string
                                let version = match dep_value {
                                    toml::Value::String(ref mut s) => Some(s),
                                    toml::Value::Table(ref mut table) => {
                                        if let Some(version) = table.get_mut("version") {
                                            if let toml::Value::String(ref mut s) = version {
                                                Some(s)
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                };
                                // Replace the version string with the newest version
                                if let Some(version) = version {
                                    // Only if the current version is a wildcard or the --upgrade flag was used
                                    if version == "*" || upgrade {
                                        match get_version(crate_name) {
                                            // Don't change versions that are already the newest
                                            Ok(ver) => if version != &ver {
                                                println!(
                                                    "{}: {} -> {}",
                                                    crate_name.bright_yellow(),
                                                    version.cyan(),
                                                    ver.bright_cyan()
                                                );
                                                if version == "*" {
                                                    stabilized += 1;
                                                } else if upgrade {
                                                    upgraded += 1;
                                                }
                                                *version = ver;
                                            },
                                            Err(e) => println!("{}", e),
                                        }
                                    }
                                }
                            }
                            if stabilized > 0 {
                                println!(
                                    "{} {} dependenc{}",
                                    "Stabilized".bright_green(),
                                    stabilized,
                                    if stabilized == 1 { "y" } else { "ies" }
                                );
                            }
                            if upgraded > 0 {
                                println!(
                                    "{} {} dependenc{}",
                                    "Updgraded".bright_green(),
                                    upgraded,
                                    if stabilized == 1 { "y" } else { "ies" }
                                );
                            }
                            if stabilized == 0 && upgraded == 0 {
                                if upgrade {
                                    println!(
                                        "{}",
                                        "All dependencies are up to date".bright_green()
                                    );
                                } else {
                                    println!("{}", "All dependencies are stable".bright_green());
                                }
                            }
                        } else {
                            println!("Invalid dependencies");
                        }
                    } else {
                        println!("No dependencies");
                    }
                } else {
                    println!("Invalid Cargo.toml")
                }
                fs::write(
                    "Cargo.toml",
                    toml::to_string_pretty(&cargo_toml_value).unwrap(),
                ).unwrap_or_else(|e| println!("{}", e));
            }
            Err(e) => println!("{}", e),
        },
        Err(e) => println!("{}", e),
    }
}
