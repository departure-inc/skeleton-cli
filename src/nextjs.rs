use curl::easy::Easy;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

pub fn init_next() {
    println!("= init next");

    // create Makefile
    if Path::new("Makefile").exists() {
        println!("! Makefile already exists.");
    } else {
        let mut handle = Easy::new();
        handle
            .url(
                "https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/nextjs/Makefile",
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

    // create Dockerfile
    if Path::new("Dockerfile").exists() {
        println!("! Dockerfile already exists.");
    } else {
        let mut handle = Easy::new();
        handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/nextjs/Dockerfile").unwrap();
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
        handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/nextjs/compose.yml").unwrap();
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

    // create .eslintrc
    if Path::new(".eslintrc.json").exists() {
        println!("+ .eslintrc.json force overwrite.");
        let mut handle = Easy::new();
        handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/nextjs/.eslintrc.json").unwrap();
        handle
            .write_function(|data| {
                let mut outfile = File::create(".eslintrc.json").unwrap();
                outfile.write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
        handle.perform().unwrap();
    }

    // create openapi.yaml
    if Path::new("openapi.yaml").exists() {
        println!("! openapi.yaml already exists.");
    } else {
        let mut handle = Easy::new();
        handle.url("https://raw.githubusercontent.com/departure-inc/skeleton-cli/main/templates/nextjs/openapi.yaml").unwrap();
        handle
            .write_function(|data| {
                let mut outfile = File::create("openapi.yaml").unwrap();
                outfile.write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();
        handle.perform().unwrap();
        println!("! openapi.yaml created. Please edit it to your needs.")
    }

    // create .env.example
    println!("* create .env.example");
    File::create(".env.example").expect("create .env.example failed");

    // add .gitignore
    println!("+ append .gitignore");
    let path = Path::new(".gitignore");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(file, ".env").unwrap();

    // create dir
    println!("* create src/components dir");
    if let Err(e) = fs::create_dir_all("src/components") {
        println!("! create components dir failed: {:?}", e.kind())
    }

    println!("* create src/interfaces dir");
    if let Err(e) = fs::create_dir_all("src/interfaces") {
        println!("! create interfaces dir failed: {:?}", e.kind())
    }

    println!("* create src/lib dir");
    if let Err(e) = fs::create_dir_all("src/lib") {
        println!("! create lib dir failed: {:?}", e.kind())
    }

    println!("* create src/models dir");
    if let Err(e) = fs::create_dir_all("src/models") {
        println!("! create models dir failed: {:?}", e.kind())
    }

    println!("* create src/repositories dir");
    if let Err(e) = fs::create_dir_all("src/repositories") {
        println!("! create repository dir failed: {:?}", e.kind())
    }

    println!("* create src/tests dir");
    if let Err(e) = fs::create_dir_all("src/tests") {
        println!("! create tests dir failed: {:?}", e.kind())
    }

    println!("* create src/utils dir");
    if let Err(e) = fs::create_dir_all("src/utils") {
        println!("! create utils dir failed: {:?}", e.kind())
    }

    println!("* create src/app/api dir");
    if let Err(e) = fs::create_dir_all("src/app/api") {
        println!("! create api dir failed: {:?}", e.kind())
    }

    // add package.json
    println!("+ append package.json");
    let file = File::open("package.json").unwrap();
    let reader = BufReader::new(file);
    let mut data: Value = serde_json::from_reader(reader).unwrap();
    data["scripts"]["lint"] = "eslint --fix --ext .jsx,.js,.tsx,.ts .".into();
    data["scripts"]["lint.fix"] = "prettier --write \"src/**/*.{js,ts,tsx}\" && next lint".into();
    data["scripts"]["test"] = "jest --env=jsdom --verbose".into();

    let json = serde_json::to_string_pretty(&data).unwrap();
    let mut file = File::create("package.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("= init next finished.");
}

pub fn help_next() {
    println!("? help nextjs commands");
    println!("  next init");
    println!("  next help");
}
