use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub value: String,
    pub lexeme: String,
    pub line: i32,
    pub initial_position: i32,
    pub final_position: i32,
}

impl Token {
    pub fn new(
        value: &str,
        lexeme: &str,
        line: i32,
        initial_position: i32,
        final_position: i32,
    ) -> Token {
        Token {
            value: value.to_string(),
            lexeme: lexeme.to_string(),
            line,
            initial_position,
            final_position,
        }
    }
}

/**
 Lexer processes and returns Tokens of a given text
*/
pub fn tokenize_code(code_text: String) -> Vec<Token> {
    let pattern = r"(?P<FN_PROGRAM>\bfn\b)|(?P<MAIN_PROGRAM>\bmain\b)|(?P<LESS_THAN_OR_EQUAL><=)|(?P<GREATER_THAN_OR_EQUAL>>=)|(?P<LEFT_PARENTHESIS>\()|(?P<RIGHT_PARENTHESIS>\))|(?P<PLUS>\+)|(?P<LEFT_SQUARE_BRACKET>\[)|(?P<RIGHT_SQUARE_BRACKET>\])|(?P<LEFT_CURLY_BRACE>\{)|(?P<RIGHT_CURLY_BRACE>\})|(?P<MULTIPLY>\*)|(?P<DIVIDE>\/)|(?P<SUBTRACT>\-)|(?P<MODULUS>%)|(?P<EXPONENT>\^)|(?P<GREATER_THAN>>)|(?P<LESS_THAN><)|(?P<EQUAL>=)|(?P<SEMICOLON>;)|(?P<IF>\bif\b)|(?P<FOR>\bfor\b)|(?P<WHILE>\bwhile\b)|(?P<PRINT>\bprint\b)|(?P<VARIABLE>[a-zA-Z_]\w*)|(?P<NUMBER>\d+)";
    let re = Regex::new(pattern).unwrap();

    let mut tokens = Vec::new();

    for cap in re.captures_iter(&code_text) {
        for name in re.capture_names() {
            if let Some(name) = name {
                if let Some(matched) = cap.name(name) {
                    let lexeme = matched.as_str();
                    let start = matched.start();
                    let end = matched.end();

                    // Calculate line number
                    let line = code_text[..start].matches('\n').count() + 1;

                    tokens.push(Token {
                        value: name.to_string(),
                        lexeme: lexeme.to_string(),
                        line: line as i32,
                        initial_position: start as i32,
                        final_position: end as i32,
                    });
                }
            }
        }
    }

    // Print tokens for debugging
    for token in &tokens {
        println!(
            "{}: '{}' [{}-{}] on line {}",
            token.value, token.lexeme, token.initial_position, token.final_position, token.line
        );
    }

    tokens.push(Token::new("$", "$", 0, 0, 0));
    tokens
}
