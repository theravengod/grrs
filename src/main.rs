use std::{env, fs};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect(); // use : grrs -- arg1 arg2
    let (file_path, search_criteria) = parse_args(&args).expect(
        &*"There was a problem parsing args".red()
    );

    println!("Searching in: {}", file_path.yellow());
    println!("Searching for: {}", &search_criteria);

    let mut files: Vec<PathBuf> = Vec::new();
    read_directory(file_path, &mut files);

    for file in files {
        matches_term_in_file(file, &search_criteria);
    }
}

fn parse_args(args: &Vec<String>) -> Result<(String, String), String> {
    if args.len() < 3 {
        eprintln!("{}", "Not enough params !".red());
        std::process::exit(1);
    }

    let file_path =
        if args.get(1).is_none() || args.get(1).unwrap().is_empty() {
            eprintln!("{}", "No path provided !".red());
            std::process::exit(1);
        } else {
            Some(args[1].clone())
        };

    let search_criteria =
        if args.get(2).is_none() || args.get(2).unwrap().is_empty() {
            eprintln!("{}", "No search criteria provided !".red());
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

fn read_directory(directory_path: String, files: &mut Vec<PathBuf>) {
    let paths = fs::read_dir(&directory_path);

    paths.unwrap().for_each(|f| {
        let current = f.unwrap();
        if current.file_type().unwrap().is_dir() {
            read_directory(current.path().into_os_string().into_string().unwrap(), files);
        } else {
            files.push(current.path());
        }
    });
}

fn matches_term_in_file(file_path: PathBuf, search_term: &String) {
    let file = std::fs::File::open(&file_path);
    let file_abs_path = &file_path.into_os_string().into_string().unwrap();
    match file {
        Ok(f) => {
            let buff = BufReader::new(f);
            let mut counter = 0;
            for line in buff.lines() {
                match line {
                    Ok(text_line) => {
                        if text_line.contains(search_term.as_str()) {
                            println!("File {} matches line #{}: {}", file_abs_path, format!("{}", counter).green(), text_line)
                        }
                    }
                    Err(_) => {}
                }
                counter = counter + 1
            }
        }
        Err(_) => {
            eprintln!("Can't look into file {}", file_abs_path)
        }
    }
}