use std::{env, fs};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect(); // use : grrs -- arg1 arg2
    let (file_path, search_criteria) = parse_args(&args).expect(
        "There was a problem parsing args"
    );

    println!("Searching in: {}", file_path);
    println!("Searching for: {}", search_criteria);

    let files = match read_directory(file_path) {
        Ok(files) => files,
        Err(err_msg) => {
            eprintln!("{}", err_msg);
            std::process::exit(1)
        }
    };
    
    for file in files {
        println!("Found: {}", file.display())
    }
}

fn parse_args(args: &Vec<String>) -> Result<(String, String), String> {
    if args.len() < 3 {
        eprintln!("Not enough params !");
        std::process::exit(1);
    }

    let file_path =
        if args.get(1).is_none() || args.get(1).unwrap().is_empty() {
            eprintln!("No path provided !");
            std::process::exit(1);
        } else {
            Some(args[1].clone())
        };

    let search_criteria =
        if args.get(2).is_none() || args.get(2).unwrap().is_empty() {
            eprintln!("No search criteria provided !");
            std::process::exit(1);
        } else {
            Some(args[2].clone())
        };

    if file_path.is_none() || search_criteria.is_none() {
        Err(String::from("Invalid params"))
    } else {
        Ok((file_path.unwrap(), search_criteria.unwrap()))
    }
}

fn read_directory(directory_path: String) -> Result<Vec<PathBuf>, String> {
    let paths = fs::read_dir(&directory_path);

    let mut files = Vec::new();
    paths.unwrap().for_each(|f| {
       files.push(f.unwrap().path());
    });

    if !files.is_empty() {
        Ok(files)
    } else {
        Err(format!("Could not read directory {}", &directory_path))
    }
}