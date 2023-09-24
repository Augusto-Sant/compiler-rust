use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
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

fn setup_states() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut states: HashMap<&'static str, HashMap<&'static str, &'static str>> = HashMap::new();

    states.insert(
        "start",
        [
            ("letter", "identifier"),
            ("i", "keyword_if"),
            ("f", "keyword_for"),
            ("w", "keyword_while"),
            ("number", "numeric_literal"),
            ("(", "left_parenthesis"),
            (")", "right_parenthesis"),
            ("+", "plus_operator"),
            ("[", "left_square_bracket"),
            ("]", "right_square_bracket"),
            ("{", "left_curly_brace"),
            ("}", "right_curly_brace"),
            ("*", "multiply_operator"),
            ("/", "divide_operator"),
            ("-", "subtract_operator"),
            ("%", "modulus_operator"),
            ("^", "exponent_operator"),
            (">", "greater_than_operator"),
            ("<", "less_than_operator"),
            ("=", "assignment_operator"),
            (";", "semicolon"),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    states.insert(
        "identifier",
        [("letter", "identifier"), ("number", "identifier")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_if",
        [("letter", "identifier"), ("f", "keyword_if_follow")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_if_follow",
        [("letter", "identifier"), ("number", "identifier")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_for",
        [("letter", "identifier"), ("o", "keyword_for_o")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_for_o",
        [("letter", "identifier"), ("r", "keyword_for_r")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_for_r",
        [("letter", "identifier"), ("number", "identifier")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while",
        [("letter", "identifier"), ("h", "keyword_while_h")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while_h",
        [("letter", "identifier"), ("i", "keyword_while_i")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while_i",
        [("letter", "identifier"), ("l", "keyword_while_l")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while_l",
        [("letter", "identifier"), ("e", "keyword_while_e")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "keyword_while_e",
        [("letter", "identifier"), ("number", "identifier")]
            .iter()
            .cloned()
            .collect(),
    );

    states.insert(
        "numeric_literal",
        [("number", "numeric_literal")].iter().cloned().collect(),
    );

    states.insert("right_parenthesis", HashMap::new());
    states.insert("plus_operator", HashMap::new());
    states.insert("left_parenthesis", HashMap::new());
    states.insert("left_square_bracket", HashMap::new());
    states.insert("right_square_bracket", HashMap::new());
    states.insert("left_curly_brace", HashMap::new());
    states.insert("right_curly_brace", HashMap::new());
    states.insert("multiply_operator", HashMap::new());
    states.insert("modulus_operator", HashMap::new());
    states.insert("divide_operator", HashMap::new());
    states.insert("exponent_operator", HashMap::new());

    let mut greater_than_operator = HashMap::new();
    greater_than_operator.insert("=", "greater_than_or_equal_operator");
    states.insert("greater_than_operator", greater_than_operator);

    states.insert("greater_than_or_equal_operator", HashMap::new());

    let mut less_than_operator = HashMap::new();
    less_than_operator.insert("=", "less_than_or_equal_operator");
    states.insert("less_than_operator", less_than_operator);

    states.insert("less_than_or_equal_operator", HashMap::new());

    let mut assignment_operator = HashMap::new();
    assignment_operator.insert("=", "equality_operator");
    states.insert("assignment_operator", assignment_operator);

    states.insert("equality_operator", HashMap::new());
    states.insert("semicolon", HashMap::new());

    states
}

fn is_numeric_string(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn is_alphabetic_string(s: &str) -> bool {
    s.chars().all(|c| c.is_alphabetic())
}

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

        // println!("Full Lexeme: {}",full_lexeme);

        // println!("Symbol: {}, Current State: {}", symbol, current_state);

        if go_next {
            symbol_vec.remove(0);
        }
    }
    tokens.push(Token::new(current_state, full_lexeme.as_str(), 0, 0, 0));

    // println!("Result: {}", current_state);

    tokens
}
