use std::io::{self, BufRead, Write};
use std::{env, fs, process};

struct Lox {
    had_error: bool
}

impl Lox {
    fn new() -> Self {
        Lox { had_error: false }
    }

    fn run_file(&mut self, path: &String) {
        let file = fs::read_to_string(path).expect("Is a proper file");
        self.run(&file);
        if (self.had_error) {
            process::exit(65);
        }

    }

    fn run_prompt(&mut self) {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let input = io::stdin();
        let mut stream = input.lock();

        while let Ok(n) = stream.read_line(&mut line) {
            if n == 0 {
                println!("breaking");
                break;
            } else {
                self.run(&line);
                self.had_error = false;
            }

            print!("> ");
            io::stdout().flush().unwrap();
        }
    }

    fn run(&mut self, source: &String) {
        todo!();
    }

    fn error(&mut self, line: i64, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: i64, what: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, what, message); 
        self.had_error = true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
