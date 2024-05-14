use std::env;
use std::fs;

extern crate clap;
use clap::{App, Arg};

mod parse_ansi_text;

fn main() {
    let matches = App::new("My Test Program")
        .version("1.0.0")
        .author("Raz Luvaton")
        .about("Parse ANSI text")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("file to read"))
        // TODO - add initial span
        // TODO - add from line
        // .arg(Arg::with_name("num")
        //     .short("n")
        //     .long("number")
        //     .takes_value(true)
        //     .help("Five less than your favorite number"))
        .get_matches();
    
    

    let file_path = matches.value_of("file").unwrap_or("../examples/2-lines.ans");
    println!("The file passed is: {}", file_path);
    // 
    // let num_str = matches.value_of("num");
    // match num_str {
    //     None => println!("No idea what your favorite number is."),
    //     Some(s) => {
    //         match s.parse::<i32>() {
    //             Ok(n) => println!("Your favorite number must be {}.", n + 5),
    //             Err(_) => println!("That's not a number! {}", s),
    //         }
    //     }
    // }
    
    println!("Current dir {}", std::env::current_dir().unwrap().display());
    // 
    // let args: Vec<String> = env::args().collect();
    // 
    // let file_path: String;
    // if args.len() == 2 {
    //     file_path = args[1].clone();
    // } else {
    //     file_path = get_file_path_in_current_dir("../examples/2-lines.ans");
    // }

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let spans = parse_ansi_text::parse_ansi_text(&contents);

    //Print the parsed output
    for span in spans {
        println!("{:?}", span);
    }
}

fn get_file_path_in_current_dir(file_name: &str) -> String {
    env::current_dir().unwrap().as_path().join(file_name).to_str().unwrap().to_string()
}
