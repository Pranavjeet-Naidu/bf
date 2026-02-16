use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Add(usize),
    Sub(usize),
    Right(usize),
    Left(usize),
    Read,
    Write,
    BeginLoop,
    EndLoop,
}

#[wasm_bindgen]
pub fn transpile_brainfuck_to_c(code: &str) -> String {
    let tokens = tokenize(code);

    if !validate_brackets(&tokens) {
        return String::from("Error: Unbalanced brackets in the code");
    }

    generate(&tokens)
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' => {
                let mut count = 1usize;
                while let Some(&'+') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Add(count));
            }
            '-' => {
                let mut count = 1usize;
                while let Some(&'-') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Sub(count));
            }
            '>' => {
                let mut count = 1usize;
                while let Some(&'>') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Right(count));
            }
            '<' => {
                let mut count = 1usize;
                while let Some(&'<') = chars.peek() {
                    chars.next();
                    count += 1;
                }
                tokens.push(Token::Left(count));
            }
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::BeginLoop),
            ']' => tokens.push(Token::EndLoop),
            _ => {} // Ignore non-BF characters
        }
    }
    tokens
}

pub fn validate_brackets(tokens: &[Token]) -> bool {
    let mut depth: i32 = 0;

    for token in tokens {
        match token {
            Token::BeginLoop => depth += 1,
            Token::EndLoop => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }

    depth == 0
}

pub fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(
        "#include <stdio.h>\n\n\
         int main() {\n\
         \x20   char tape[30000] = {0};\n\
         \x20   char *ptr = tape;\n\n",
    );

    let mut indent_level: usize = 1;

    for token in tokens {
        let indent = "    ".repeat(indent_level);
        match token {
            Token::Add(n) => output.push_str(&format!("{}*ptr += {};\n", indent, n)),
            Token::Sub(n) => output.push_str(&format!("{}*ptr -= {};\n", indent, n)),
            Token::Right(n) => {
                output.push_str(&format!("{}ptr += {};\n", indent, n));
                output.push_str(&format!(
                    "{}if (ptr >= tape + sizeof(tape)) {{ fprintf(stderr, \"Error: Pointer out of bounds\\n\"); return 1; }}\n",
                    indent, 
                ));
            }
            Token::Left(n) => {
                output.push_str(&format!("{}ptr -= {};\n", indent, n));
                output.push_str(&format!(
                    "{}if (ptr < tape) {{ fprintf(stderr, \"Error: Pointer out of bounds\\n\"); return 1; }}\n",
                    indent,
                ));
            }
            Token::Read => output.push_str(&format!("{}*ptr = getchar();\n", indent)),
            Token::Write => output.push_str(&format!("{}putchar(*ptr);\n", indent)),
            Token::BeginLoop => {
                output.push_str(&format!("{}while (*ptr) {{\n", indent));
                indent_level += 1;
            }
            Token::EndLoop => {
                indent_level = indent_level.saturating_sub(1).max(1);
                let indent = "    ".repeat(indent_level);
                output.push_str(&format!("{}}}\n", indent));
            }
        }
    }

    output.push_str("\n    return 0;\n}\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let tokens = tokenize("++--><,.[");
        assert_eq!(tokens, vec![
            Token::Add(2),
            Token::Sub(2),
            Token::Right(1),
            Token::Left(1),
            Token::Read,
            Token::Write,
            Token::BeginLoop,
        ]);
    }

    #[test]
    fn test_tokenize_ignores_non_bf() {
        let tokens = tokenize("hello + world");
        assert_eq!(tokens, vec![Token::Add(1)]);
    }

    #[test]
    fn test_run_length_no_overflow() {
        // 300 '+' chars — previously would overflow u8 to 44
        let code: String = std::iter::repeat('+').take(300).collect();
        let tokens = tokenize(&code);
        assert_eq!(tokens, vec![Token::Add(300)]);
    }

    #[test]
    fn test_validate_brackets_balanced() {
        let tokens = tokenize("[++[--]++]");
        assert!(validate_brackets(&tokens));
    }

    #[test]
    fn test_validate_brackets_unbalanced() {
        assert!(!validate_brackets(&tokenize("[[")));
        assert!(!validate_brackets(&tokenize("][")));
        assert!(!validate_brackets(&tokenize("]")));
    }

    #[test]
    fn test_generate_hello_world() {
        let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>[<]<-]>>.";
        let tokens = tokenize(code);
        let c_code = generate(&tokens);
        assert!(c_code.contains("#include <stdio.h>"));
        assert!(c_code.contains("while (*ptr) {"));
        assert!(c_code.contains("return 0;"));
    }

    #[test]
    fn test_nested_indentation() {
        let tokens = tokenize("[[-]]");
        let c_code = generate(&tokens);
        // Inner loop should be indented more than outer
        assert!(c_code.contains("        while (*ptr) {\n"));
        assert!(c_code.contains("        }\n"));
    }

    #[test]
    fn test_transpile_error_on_unbalanced() {
        let result = transpile_brainfuck_to_c("[[+]");
        assert!(result.starts_with("Error:"));
    }
}