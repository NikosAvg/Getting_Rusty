/*
 Rust clone of Echo,
 a cli tool that prints to STDOUT the arguments
 that are passed to it.
*/

/*
  {:?} to format the debug view of the arguments
  {:#?} to include newlines and indentations to help
me read the output. This is called pretty-printing because, well, it’s
prettier.
*/
use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Nikos Avgoustis")
        .about("Echo written in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    // Onew way to write an if
    // let mut ending = "\n";
    // if omit_newline {
    //     ending = "";
    // }
    // A Rustic way to write if (if in Rust is an expression and not a statement)
    // let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
