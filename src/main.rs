use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <brainfuck-code>", args[0]);
        eprintln!("       {} -f <filename>", args[0]);
        std::process::exit(1);
    }

    let code = if args[1] == "-f" {
        if args.len() < 3 {
            eprintln!("Missing filename after -f");
            std::process::exit(1);
        }
        match fs::read_to_string(&args[2]) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        args[1].clone()
    };

    let c_code = bf::transpile_brainfuck_to_c(&code);

    if c_code.starts_with("Error:") {
        eprintln!("{}", c_code);
        std::process::exit(1);
    }

    // If a third arg (or second after -f <file>) is given, use it as output filename
    let output_file = if args[1] == "-f" {
        args.get(3).map(|s| s.as_str())
    } else {
        args.get(2).map(|s| s.as_str())
    };

    match output_file {
        Some(filename) => {
            if let Err(e) = fs::write(filename, &c_code) {
                eprintln!("Failed to write to {}: {}", filename, e);
                std::process::exit(1);
            }
            eprintln!("Wrote C code to {}", filename);
        }
        None => {
            print!("{}", c_code);
        }
    }
}
