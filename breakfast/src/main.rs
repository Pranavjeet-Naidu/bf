use std::env;
use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add(u8),    // + (with count)
    Sub(u8),    // - (with count)
    Right(u8),  // > (with count)
    Left(u8),   // < (with count)
    Read,       // ,
    Write,      // .
    BeginLoop,  // [
    EndLoop,    // ]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <brainfuck-code> or {} -f <filename>", args[0], args[0]);
        return;
    }
    
    let code = if args[1] == "-f" {
        if args.len() < 3 {
            println!("Missing filename after -f");
            return;
        }
        match fs::read_to_string(&args[2]) {
            Ok(content) => content,
            Err(e) => {
                println!("Error reading file: {}", e);
                return;
            }
        }
    } else {
        args[1].clone()
    };
    
    let tokens = tokenize(&code);
    println!("Tokens: {:?}", tokens);
    
    if !validate_brackets(&tokens) {
        println!("Error: Unbalanced brackets in the code");
        return;
    }
    
    let generated_code = generate(&tokens);
    println!("Generated code:\n{}", generated_code);

    match save_to_file(&generated_code, "output.c") {
        Ok(_) => println!("C code written to output.c"),
        Err(e) => eprintln!("Failed to write to file: {}", e),
    }
}

fn save_to_file(code: &str, filename: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;
    use std::env;
    
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join(filename);

    println!("Saving to: {}", file_path.display());

    let mut file = File::create(filename)?;
    file.write_all(code.as_bytes())?;
    Ok(())
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                let mut count = 1;
                while let Some(&'+') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Add(count as u8));
            },
            '-' => {
                let mut count = 1;
                while let Some(&'-') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Sub(count as u8));
            },
            '>' => {
                let mut count = 1;
                while let Some(&'>') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Right(count as u8));
            },
            '<' => {
                let mut count = 1;
                while let Some(&'<') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Left(count as u8));
            },
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::BeginLoop),
            ']' => tokens.push(Token::EndLoop),
            _ => {} // Ignore other characters
        }
    }
    tokens
}

fn validate_brackets(tokens: &[Token]) -> bool {
    let mut nesting = 0;
    
    for token in tokens {
        match token {
            Token::BeginLoop => nesting += 1,
            Token::EndLoop => {
                nesting -= 1;
                if nesting < 0 {
                    return false;
                }
            },
            _ => {}
        }
    }
    
    nesting == 0
}

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from("#include <stdio.h>\n\nint main() {\n    char tape[20000] = {0};\n    char *ptr = tape;\n\n"); // Open main function

    for token in tokens {
        match token {
            Token::Add(count) => output.push_str(&format!("    *ptr += {};\n", count)),
            Token::Sub(count) => output.push_str(&format!("    *ptr -= {};\n", count)),
            Token::Right(count) => output.push_str(&format!("    ptr += {};\n    if (ptr >= tape + sizeof(tape)) {{ fprintf(stderr, \"Error: Pointer out of bounds\\n\"); return 1; }}\n", count)),
            Token::Left(count) => output.push_str(&format!("    ptr -= {};\n    if (ptr < tape) {{ fprintf(stderr, \"Error: Pointer out of bounds\\n\"); return 1; }}\n", count)),
            Token::Read => output.push_str("    *ptr = getchar();\n"),
            Token::Write => output.push_str("    putchar(*ptr);\n"),
            Token::BeginLoop => output.push_str("    while (*ptr) {\n"),
            Token::EndLoop => output.push_str("    }\n"),
        }
    }

    output.push_str("\n    return 0;\n}\n"); // Close main function properly
    output
}
