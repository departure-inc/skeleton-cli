use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn init_rails() {
    println!("= init rails");

    // add Gemfile
    let path = Path::new("Gemfile");
    if !path.exists() {
        return println!("! Gemfile is not found.");
    }

    let mut file = OpenOptions::new().append(true).open(path).unwrap();

    writeln!(
        file,
        "gem 'skeleton_generator', github: 'departure-inc/skeleton-generator'"
    )
    .unwrap();

    println!("! Gemfile updated.");
    println!("= init rails finished.");
}

//
// # 新しいRailsアプリの作成(WebApp)
// $ rails new {project-app-name} -d postgresql -T -B -a propshaft -c tailwind
pub fn new_rails(name: &str) {
    println!("= rails new {}", name);

    let mut bundle = Command::new("rails");
    bundle
        .arg("new")
        .arg(name)
        .args(["-d", "postgresql"])
        .arg("-T")
        .arg("-B")
        .args(["-a", "propshaft"])
        .args(["-c", "tailwind"]);
    let output = bundle.output().expect("failed to execute rails command");

    io::stdout().write_all(&output.stdout).unwrap();
    println!("= rails new project created.");
}

//
// # 新しいRailsアプリの作成(APIモード)
// $ rails new {project-app-name} --api -d postgresql -T -B
pub fn api_rails(name: &str) {
    println!("= rails new {} --api", name);

    let mut bundle = Command::new("rails");
    bundle
        .arg("new")
        .arg(name)
        .arg("--api")
        .args(["-d", "postgresql"])
        .arg("-T")
        .arg("-B");
    let output = bundle.output().expect("failed to execute rails command");

    io::stdout().write_all(&output.stdout).unwrap();
    println!("= rails api project created.");
}

pub fn help_rails() {
    println!("? help rails commands");
    println!("  rails init");
    println!("  rails help");
    println!("  rails new project-name");
    println!("  rails api project-name");
}
