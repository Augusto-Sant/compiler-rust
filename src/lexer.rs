use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Token {
    value: String,
    lexeme: String,
    line: i32,
    initial_position: i32,
    final_position: i32,
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
 Makes the automaton for lexer, uppercase are final states
*/
fn setup_states() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut states: HashMap<&'static str, HashMap<&'static str, &'static str>> = HashMap::new();

    states.insert(
        "start",
        [
            ("letter", "VARIABLE"),
            ("i", "keyword_if"),
            ("f", "keyword_for"),
            ("w", "keyword_while"),
            ("p", "keyword_print"),
            ("number", "NUMBER"),
            ("(", "LEFT_PARENTHESIS"),
            (")", "RIGHT_PARENTHESIS"),
            ("+", "PLUS"),
            ("[", "LEFT_SQUARE_BRACKET"),
            ("]", "RIGHT_SQUARE_BRACKET"),
            ("{", "LEFT_CURLY_BRACE"),
            ("}", "RIGHT_CURLY_BRACE"),
            ("*", "MULTIPLY"),
            ("/", "DIVIDE"),
            ("-", "SUBTRACT"),
            ("%", "MODULUS"),
            ("^", "EXPONENT"),
            (">", "GREATER_THAN"),
            ("<", "LESS_THAN"),
            ("=", "EQUAL"),
            (";", "SEMICOLON"),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    states.insert(
        "VARIABLE",
        [("letter", "VARIABLE"), ("number", "VARIABLE")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_print",
        [
            ("letter", "VARIABLE"),
            ("number", "VARIABLE"),
            ("r", "keyword_print_r"),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    states.insert(
        "keyword_print_r",
        [
            ("letter", "VARIABLE"),
            ("number", "VARIABLE"),
            ("i", "keyword_print_i"),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    states.insert(
        "keyword_print_i",
        [
            ("letter", "VARIABLE"),
            ("number", "VARIABLE"),
            ("n", "keyword_print_n"),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    states.insert(
        "keyword_print_n",
        [
            ("letter", "VARIABLE"),
            ("number", "VARIABLE"),
            ("t", "PRINT"),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    states.insert(
        "PRINT",
        [("letter", "VARIABLE"), ("number", "VARIABLE")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_if",
        [("letter", "VARIABLE"), ("f", "IF")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "IF",
        [("letter", "VARIABLE"), ("number", "VARIABLE")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_for",
        [
            ("letter", "VARIABLE"),
            ("o", "keyword_for_o"),
            ("n", "FN_PROGRAM"),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    states.insert(
        "FN_PROGRAM",
        [("letter", "VARIABLE"), ("number", "VARIABLE")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_for_o",
        [("letter", "VARIABLE"), ("r", "FOR")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "FOR",
        [("letter", "VARIABLE"), ("number", "VARIABLE")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while",
        [("letter", "VARIABLE"), ("h", "keyword_while_h")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while_h",
        [("letter", "VARIABLE"), ("i", "keyword_while_i")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while_i",
        [("letter", "VARIABLE"), ("l", "keyword_while_l")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while_l",
        [("letter", "VARIABLE"), ("e", "WHILE")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "WHILE",
        [("letter", "VARIABLE"), ("number", "VARIABLE")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert("NUMBER", [("number", "NUMBER")].iter().cloned().collect());

    states.insert("RIGHT_PARENTHESIS", HashMap::new());
    states.insert("PLUS", HashMap::new());
    states.insert("LEFT_PARENTHESIS", HashMap::new());
    states.insert("LEFT_SQUARE_BRACKET", HashMap::new());
    states.insert("RIGHT_SQUARE_BRACKET", HashMap::new());
    states.insert("LEFT_CURLY_BRACE", HashMap::new());
    states.insert("RIGHT_CURLY_BRACE", HashMap::new());
    states.insert("MULTIPLY", HashMap::new());
    states.insert("MODULUS", HashMap::new());
    states.insert("DIVIDE", HashMap::new());
    states.insert("SUBTRACT", HashMap::new());
    states.insert("EXPONENT", HashMap::new());

    let mut greater_than_operator = HashMap::new();
    greater_than_operator.insert("=", "GREATER_THAN_OR_EQUAL");
    states.insert("GREATER_THAN", greater_than_operator);

    states.insert("GREATER_THAN_OR_EQUAL", HashMap::new());

    let mut less_than_operator = HashMap::new();
    less_than_operator.insert("=", "LESS_THAN_OR_EQUAL");
    states.insert("LESS_THAN", less_than_operator);

    states.insert("LESS_THAN_OR_EQUAL", HashMap::new());

    let mut assignment_operator = HashMap::new();
    assignment_operator.insert("=", "EQUAL");
    states.insert("EQUAL", assignment_operator);

    states.insert("EQUAL", HashMap::new());
    states.insert("SEMICOLON", HashMap::new());

    states
}

fn is_numeric_string(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn is_alphabetic_string(s: &str) -> bool {
    s.chars().all(|c| c.is_alphabetic())
}

/**
 Lexer processes and returns Tokens of a given text
*/
pub fn tokenize_code(code_text: &str) -> Vec<Token> {
    let map = setup_states();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_state = "start";
    let mut symbol_vec = Vec::new();
    let mut full_lexeme = String::new();
    let mut go_next = true;
    let mut string_symbol = "".to_string();
    let mut symbol;

    for character in code_text.chars() {
        if !character.is_whitespace() {
            let letter_str = character.to_string();
            symbol_vec.push(letter_str);
        }
    }

    while !symbol_vec.is_empty() {
        if go_next {
            if let Some(first_symbol) = symbol_vec.get(0) {
                string_symbol = first_symbol.clone().to_string();
            }
        }
        symbol = string_symbol.as_str();

        if let Some(inner_map) = map.get(current_state) {
            if let Some(next_state) = inner_map.get(symbol) {
                current_state = next_state;
                full_lexeme.push_str(symbol);
                go_next = true;
            } else if is_alphabetic_string(symbol) && inner_map.contains_key("letter") {
                current_state = inner_map.get("letter").unwrap();
                full_lexeme.push_str(symbol);
                go_next = true;
            } else if is_numeric_string(symbol) && inner_map.contains_key("number") {
                current_state = inner_map.get("number").unwrap();
                full_lexeme.push_str(symbol);
                go_next = true;
            } else {
                tokens.push(Token::new(current_state, full_lexeme.as_str(), 0, 0, 0));
                // println!(">> TOKEN {} ", full_lexeme);
                current_state = "start";
                full_lexeme = "".to_string();
                if inner_map.contains_key(symbol) {
                    go_next = true;
                } else {
                    go_next = false;
                }
            }
        }
        if go_next {
            symbol_vec.remove(0);
        }
    }
    tokens.push(Token::new(current_state, full_lexeme.as_str(), 0, 0, 0));
    // println!("Result: {}", current_state);
    tokens
}
