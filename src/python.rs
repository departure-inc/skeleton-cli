use curl::easy::Easy;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn init_python() {
    println!("= init python");

    // create Makefile
    if Path::new("Makefile").exists() {
        println!("! Makefile already exists.");
    } else {
        let mut handle = Easy::new();
        handle
            .url(
                "https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/fastapi/Makefile",
            )
            .unwrap();
        handle
            .write_function(|data| {
                let mut outfile = File::create("Makefile").unwrap();
                outfile.write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
        handle.perform().unwrap();
        println!("! Makefile created. Please edit it to your needs.")
    }

    // create Procfile
    if Path::new("Procfile").exists() {
        println!("! Procfile already exists.");
    } else {
        let mut handle = Easy::new();
        handle
            .url(
                "https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/fastapi/Procfile",
            )
            .unwrap();
        handle
            .write_function(|data| {
                let mut outfile = File::create("Procfile").unwrap();
                outfile.write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
        handle.perform().unwrap();
        println!("! Procfile created. Please edit it to your needs.")
    }

    // create Dockerfile
    if Path::new("Dockerfile").exists() {
        println!("! Dockerfile already exists.");
    } else {
        let mut handle = Easy::new();
        handle
            .url(
                "https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/fastapi/Dockerfile",
            )
            .unwrap();
        handle
            .write_function(|data| {
                let mut outfile = File::create("Dockerfile").unwrap();
                outfile.write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
        handle.perform().unwrap();
        println!("! Dockerfile created. Please edit it to your needs.")
    }

    // create compose.yml
    if Path::new("compose.yml").exists() {
        println!("! compose.yml already exists.");
    } else {
        let mut handle = Easy::new();
        handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/fastapi/compose.yml").unwrap();
        handle
            .write_function(|data| {
                let mut outfile = File::create("compose.yml").unwrap();
                outfile.write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
        handle.perform().unwrap();
        println!("! compose.yml created. Please edit it to your needs.")
    }

    // create app.py
    if Path::new("app.py").exists() {
        println!("! app.py already exists.");
    } else {
        let mut handle = Easy::new();
        handle
            .url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/fastapi/app.py")
            .unwrap();
        handle
            .write_function(|data| {
                let mut outfile = File::create("app.py").unwrap();
                outfile.write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
        handle.perform().unwrap();
        println!("! app.py created. Please edit it to your needs.")
    }

    // create src dir
    println!("+ create src dir");
    if let Err(e) = fs::create_dir_all("src") {
        println!("! create src dir failed: {:?}", e.kind())
    }
    File::create("src/.keep").expect("create .keep failed");

    // create migration dir
    println!("+ create migrations dir");
    if let Err(e) = fs::create_dir_all("migrations") {
        println!("! create migrations dir failed: {:?}", e.kind())
    }
    File::create("migrations/.keep").expect("create .keep failed");

    // add Pipenv
    println!("+ add Pipenv");
    Command::new("pip install pipenv")
        .output()
        .expect("failed to execute process");
    Command::new("pipenv install fastapi uvicorn[standard]")
        .output()
        .expect("failed to execute process");
    Command::new("pipenv install --dev flake8 autopep8")
        .output()
        .expect("failed to execute process");

    println!("= init python finished.");
}

pub fn help_python() {
    println!("? help python commands");
    println!("  python init");
    println!("  python help");
}
