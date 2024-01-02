/*
Rust clone of cat.
*/

use clap::{App, Arg};
use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
// A struct called Config to represent the command-line parameters of the programm
// The derive macro adds the Debug trait so the struct can be printed.

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Nikos Avgoustis")
        .about("Rust Version of Cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input Files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number each line")
                .takes_value(false)
                .conflicts_with("number_nonblank_line"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number each nonblank line")
                .takes_value(false)
                .conflicts_with("number_line"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}
pub fn read_file(filename: &str, nl: bool, nbl: bool) {
    let mut iter = 1;
    if nl == true {
        for line in read_to_string(filename).unwrap().lines() {
            println!("{} {}", iter, line.to_string());
            iter += 1;
        }
    } else if nbl == true {
        for line in read_to_string(filename).unwrap().lines() {
            if line != "" {
                println!("{} {}", iter, line.to_string());
                iter += 1;
            }
        }
    } else {
        for line in read_to_string(filename).unwrap().lines() {
            println!("{}", line.to_string());
        }
    }
}
// My solution
// pub fn run(config: Config) -> MyResult<()> {
//     for filename in config.files {
//         match open(&filename) {
//             Err(err) => eprintln!("Failed to open {}: {}", filename, err),
//             Ok(_) => read_file(&filename, config.number_lines, config.number_nonblank_lines),
//         }
//     }
//     Ok(())
// }
// Correct solution
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
