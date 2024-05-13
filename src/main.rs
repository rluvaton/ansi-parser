use std::env;
use std::fs;

mod parse_ansi_text;

fn main() {
    println!("Current dir {}", std::env::current_dir().unwrap().display());

    let args: Vec<String> = env::args().collect();

    let file_path: String;
    if args.len() == 2 {
        file_path = args[1].clone();
    } else {
        file_path = get_file_path_in_current_dir("../examples/2-lines.ans");
    }

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
