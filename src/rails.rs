use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn init_rails() {
    println!("= init rails");

    // add Gemfile
    let path = Path::new("Gemfile");
    if !path.exists() {
        return println!("! Gemfile is not found.");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(
        file,
        "gem 'skeleton_generator', github: 'departure-inc/skeleton-generator'"
    )
    .unwrap();

    println!("! Gemfile updated.");
    println!("= init rails finished.");
}

pub fn help_rails() {
    println!("? help rails commands");
    println!("  rails init");
    println!("  rails help");
}
