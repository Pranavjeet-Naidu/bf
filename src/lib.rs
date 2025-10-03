use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn transpile_brainfuck_to_c(code: &str) -> String {
    let tokens = tokenize(code);
    
    if !validate_brackets(&tokens) {
        return String::from("Error: Unbalanced brackets in the code");
    }
    
    generate(&tokens)
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