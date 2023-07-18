use curl::easy::Easy;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn init_rust() {
    println!("= init rust");

    // create Makefile
    if Path::new("Makefile").exists() {
        println!("! Makefile already exists.");
    } else {
        let mut handle = Easy::new();
        handle
            .url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/axum/Makefile")
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

    // create Dockerfile
    if Path::new("Dockerfile").exists() {
        println!("! Dockerfile already exists.");
    } else {
        let mut handle = Easy::new();
        handle
            .url(
                "https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/axum/Dockerfile",
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
        handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/axum/compose.yml").unwrap();
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

    // create .env.example
    println!("* create .env.example");
    File::create(".env.example").expect("create .env.example failed");

    // create projects dir
    println!("* create src/application dir");
    if let Err(e) = fs::create_dir_all("src/application") {
        println!("! create application dir failed: {:?}", e.kind())
    }
    File::create("src/application/mod.rs").expect("create mod.rs failed");

    println!("* create src/domain dir");
    if let Err(e) = fs::create_dir_all("src/domain") {
        println!("! create domain dir failed: {:?}", e.kind())
    }
    File::create("src/domain/mod.rs").expect("create mod.rs failed");
    // pub mod model;

    println!("* create src/domain/model dir");
    if let Err(e) = fs::create_dir_all("src/domain/model") {
        println!("! create domain/model dir failed: {:?}", e.kind())
    }
    File::create("src/domain/model/mod.rs").expect("create mod.rs failed");

    println!("* create src/infrastructure dir");
    if let Err(e) = fs::create_dir_all("src/infrastructure") {
        println!("! create infrastructure dir failed: {:?}", e.kind())
    }
    File::create("src/infrastructure/mod.rs").expect("create mod.rs failed");
    //pub mod middleware;
    //pub mod repository;
    //pub mod request;
    //pub mod response;

    println!("* create src/infrastructure/middleware dir");
    if let Err(e) = fs::create_dir_all("src/infrastructure/middleware") {
        println!(
            "! create infrastructure/middleware dir failed: {:?}",
            e.kind()
        )
    }
    File::create("src/infrastructure/middleware/mod.rs").expect("create mod.rs failed");
    //pub mod database;
    let mut handle = Easy::new();
    handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/axum/src/infrastructure/middleware/database.rs").unwrap();
    handle
        .write_function(|data| {
            let mut outfile = File::create("src/infrastructure/middleware/database.rs").unwrap();
            outfile.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
    handle.perform().unwrap();

    println!("* create src/infrastructure/repository dir");
    if let Err(e) = fs::create_dir_all("src/infrastructure/repository") {
        println!(
            "! create infrastructure/repository dir failed: {:?}",
            e.kind()
        )
    }
    File::create("src/infrastructure/repository/mod.rs").expect("create mod.rs failed");

    println!("* create src/infrastructure/request dir");
    if let Err(e) = fs::create_dir_all("src/infrastructure/request") {
        println!("! create infrastructure/request dir failed: {:?}", e.kind())
    }
    File::create("src/infrastructure/request/mod.rs").expect("create mod.rs failed");

    println!("* create src/infrastructure/response dir");
    if let Err(e) = fs::create_dir_all("src/infrastructure/response") {
        println!(
            "! create infrastructure/response dir failed: {:?}",
            e.kind()
        )
    }
    File::create("src/infrastructure/response/mod.rs").expect("create mod.rs failed");

    println!("* create src/presentation dir");
    if let Err(e) = fs::create_dir_all("src/presentation") {
        println!("! create presentation dir failed: {:?}", e.kind())
    }

    // pub mod application_controller;
    let mut handle = Easy::new();
    handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/axum/src/presentation/application_controller.rs").unwrap();
    handle
        .write_function(|data| {
            let mut outfile = File::create("src/presentation/application_controller.rs").unwrap();
            outfile.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
    handle.perform().unwrap();

    // create main.rs
    println!("* create src/main.rs");
    let mut handle = Easy::new();
    handle
        .url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/axum/src/main.rs")
        .unwrap();
    handle
        .write_function(|data| {
            let mut outfile = File::create("src/main.rs").unwrap();
            outfile.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
    handle.perform().unwrap();

    // create migration dir
    if let Err(e) = fs::create_dir_all("migrations") {
        println!("! create migrations dir failed: {:?}", e.kind())
    }
    File::create("migrations/.keep").expect("create .keep failed");

    // create test dir
    if let Err(e) = fs::create_dir_all("tests") {
        println!("! create tests dir failed: {:?}", e.kind())
    }
    File::create("tests/.keep").expect("create .keep failed");

    // TODO: executable help
    // Command::new("cargo install cargo-shuttle").output().expect("failed to execute process");
    // Command::new("cargo install cargo-watch").output().expect("failed to execute process");
    // Command::new("cargo install cargo-edit").output().expect("failed to execute process");
    // Command::new("cargo install sqlx-cli").output().expect("failed to execute process");

    // Cargo.toml
    Command::new("cargo add axum")
        .output()
        .expect("failed to execute process");
    Command::new("cargo add dotenv")
        .output()
        .expect("failed to execute process");
    Command::new("cargo add serde --features 'derive'")
        .output()
        .expect("failed to execute process");
    Command::new("cargo add sqlx --features 'runtime-tokio-rustls postgres macros chrono'")
        .output()
        .expect("failed to execute process");
    Command::new("cargo add tokio --features 'full'")
        .output()
        .expect("failed to execute process");
    Command::new("cargo add tracing")
        .output()
        .expect("failed to execute process");
    Command::new("cargo add tracing-subscriber --features 'std env-filter'")
        .output()
        .expect("failed to execute process");

    println!("= init rust finished.");
}

pub fn help_rust() {
    println!("? help rust commands");
    println!("  rust init");
    println!("  rust help");
}
