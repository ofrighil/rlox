use std::io::{self, BufRead};
use std::{env, fs, process};

fn run(source: &String) {
    todo!();
}

fn run_prompt() {
    let mut line = String::new();
    let input = io::stdin();
    let mut stream = input.lock();

    while let Ok(n) = stream.read_line(&mut line) {
        if n == 0 {
            println!("breaking");
            break;
        }
    }
}

fn run_file(path: &String) {
    let file = fs::read_to_string(path).expect("Is a proper file");
    run(&file);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
