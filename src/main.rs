use std::env;

mod flutter;
mod help;
mod nextjs;
mod python;
mod rails;
mod rust;

fn main() {
    let args: Vec<String> = env::args().collect();
    /*
    println!("{:?}", args);
    println!("Hello, world!");
    println!("The first argument is {}", args.len());
    println!("My path is {}", args[0]);
    println!("The remaining arguments are {:?}", &args[1..]);
    */

    if args.len() < 3 {
        help::help_command();
        std::process::exit(1);
    }

    let _current_path = env::current_dir().unwrap();
    let main_command = args[1].clone();
    let sub_command = args[2].clone();

    /*
    println!("current path is {}", current_path.display());
    println!("main command is {}", main_command);
    println!("sub command is {}", sub_command);
    */

    match main_command.as_str() {
        "rails" => match sub_command.as_str() {
            "init" => rails::init_rails(),
            "help" => rails::help_rails(),
            _ => not_found(&sub_command),
        },
        "next" => match sub_command.as_str() {
            "init" => nextjs::init_next(),
            "help" => nextjs::help_next(),
            _ => not_found(&sub_command),
        },
        "rust" => match sub_command.as_str() {
            "init" => rust::init_rust(),
            "help" => rust::help_rust(),
            _ => not_found(&sub_command),
        },
        "python" => match sub_command.as_str() {
            "init" => python::init_python(),
            "help" => python::help_python(),
            _ => not_found(&sub_command),
        },
        "flutter" => match sub_command.as_str() {
            "init" => flutter::init_flutter(),
            "help" => flutter::help_flutter(),
            _ => not_found(&sub_command),
        },
        "help" => help::help_command(),
        _ => not_found(&main_command),
    }
}

fn not_found(text: &str) {
    println!("! {} is not found.", text);
    println!("! example: skeleton help");
}
