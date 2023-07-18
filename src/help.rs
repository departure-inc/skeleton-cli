pub fn help_command() {
    /*
    let matches = App::new("skeleton-cli")
        .version("0.1.0")
        .author("kawashima <kawashima@dptr.jp>")
        .about("This is the skeleton command for the initial project file deployment.")
        .get_matches();
    */
    // TODO: Replaced by crate clap later.
    eprintln!("skeleton-cli 0.1.0");
    eprintln!("kawashima <kawashima@dptr.jp>");
    eprintln!();
    eprintln!("This is the skeleton command for the initial project file deployment.");
    eprintln!();
    eprintln!("See: https://github.com/departure-inc/skeleton-cli");
    eprintln!();
    eprintln!("Check skeleton/README.md on departure inc for details.");
    eprintln!("See: https://github.com/rawhide/skeleton");
    eprintln!();
    eprintln!("Usage: skeleton [OPTIONS] [ARGS]");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("  rails  : Ruby on Rails");
    eprintln!("  next   : Next.js");
    eprintln!("  rust   : Rust, Axum and SQLx");
    eprintln!("  python : Python, FastAPI and SQLAlchemy");
    eprintln!("  flutter: Flutter");
    eprintln!();
    eprintln!("ARGS:");
    eprintln!("  init  : initialize project.");
    eprintln!("  help  : help.");
}
