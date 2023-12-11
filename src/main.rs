use axum::{
    extract::Json,
    http::Response,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Instant;
use tera::{Context, Tera};

mod lexer;
mod parser;
mod semantic;
mod util;

#[derive(Deserialize)]
struct TokenRequest {
    code_text: String,
    mode: String,
}

async fn check_language(Json(body): Json<TokenRequest>) -> impl IntoResponse {
    let start_time = Instant::now();
    let tokens = lexer::tokenize_code(body.code_text.to_string());
    let tokens_processed = tokens.len();
    let (is_syntax_correct, syntax_tree) = parser::syntax_parse(tokens);
    println!("{}", is_syntax_correct);

    let mut semantic_error = None;
    if is_syntax_correct {
        if let Some(syntax_tree) = syntax_tree {
            if let Err(err) = semantic::semantic_analysis(&syntax_tree) {
                semantic_error = Some(format!(
                    "Semantic Error: {} at line {}, position {} to {}",
                    err.message, err.line, err.initial_position, err.final_position
                ));
            }
        }
    }
    let end_time = Instant::now();
    let elapsed_time = (end_time - start_time).as_secs_f64() * 1000.0;
    println!(
        "INFO: Processed and analysed syntax in tokens -> {tokens_processed} in {elapsed_time} ms"
    );
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = Context::new();
    context.insert("is_syntax_correct", &is_syntax_correct);
    if let Some(error) = semantic_error {
        context.insert("semantic_error", &error);
    }
    context.insert("code_text", &body.code_text);

    let rendered = tera.render("compiler_template.html", &context).unwrap();
    rendered.into_response()
}

async fn check_syntax(Json(body): Json<TokenRequest>) -> impl IntoResponse {
    let start_time = Instant::now();
    let tokens = lexer::tokenize_code(body.code_text.to_string());
    let tokens_processed = tokens.len();
    let (is_syntax_correct, _) = parser::syntax_parse(tokens);
    let end_time = Instant::now();
    let elapsed_time = (end_time - start_time).as_secs_f64() * 1000.0;
    println!(
        " INFO: processed and analysed syntax in tokens -> {tokens_processed} in {elapsed_time} ms"
    );

    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("is_syntax_correct", &is_syntax_correct);
    let rendered = tera.render("syntax_template.html", &context).unwrap();
    rendered.into_response()
}

async fn generate_tokens(Json(body): Json<TokenRequest>) -> impl IntoResponse {
    let start_time = Instant::now();
    let tokens = lexer::tokenize_code(body.code_text.to_string());
    let tokens_processed = tokens.len();
    let end_time = Instant::now();
    let elapsed_time = (end_time - start_time).as_secs_f64() * 1000.0;
    println!(" INFO: processed tokens -> {tokens_processed} in {elapsed_time} ms");

    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("tokens", &tokens);
    let rendered = tera.render("tokens_template.html", &context).unwrap();

    match body.mode.as_str() {
        "json" => Json(tokens).into_response(),
        "html" => rendered.into_response(),
        _ => rendered.into_response(),
    }
}

async fn index() -> impl IntoResponse {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = Context::new();
    let rendered = tera.render("index_template.html", &context).unwrap();
    Response::new(rendered)
}

async fn tab1() -> impl IntoResponse {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = Context::new();
    let rendered = tera.render("tab1.html", &context).unwrap();
    Response::new(rendered)
}

async fn tab2() -> impl IntoResponse {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = Context::new();
    let rendered = tera.render("tab2.html", &context).unwrap();
    Response::new(rendered)
}

async fn tab3() -> impl IntoResponse {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = Context::new();
    let rendered = tera.render("tab3.html", &context).unwrap();
    Response::new(rendered)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/tokens", post(generate_tokens))
        .route("/tab1", get(tab1))
        .route("/tab2", get(tab2))
        .route("/tab3", get(tab3))
        .route("/syntax", post(check_syntax))
        .route("/compile", post(check_language));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on -> http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[cfg(test)]
// mod tests {
//     use crate::lexer::{tokenize_code, Token};
//     use crate::util::read_file;
//     // use crate::parser::syntax_parse;
//
//     // #[test]
//     // fn test_slr_parser() {
//     //     let code_text = read_file("example_code.txt");
//     //     // let tokens = tokenize_code(&code_text);
//     //     // syntax_parse(tokens)
//     // }
//
//     #[test]
//     fn test_tokenize_code() {
//         let code_text = read_file("example_code.txt");
//         let tokens_expected: Vec<Token> = vec![
//             Token::new("FN_PROGRAM", "fn", 0, 0, 0),
//             Token::new("LEFT_PARENTHESIS", "(", 0, 0, 0),
//             Token::new("RIGHT_PARENTHESIS", ")", 0, 0, 0),
//             Token::new("LEFT_CURLY_BRACE", "{", 0, 0, 0),
//             Token::new("WHILE", "while", 0, 0, 0),
//             Token::new("LEFT_PARENTHESIS", "(", 0, 0, 0),
//             Token::new("VARIABLE", "x", 0, 0, 0),
//             Token::new("LESS_THAN_OR_EQUAL", "<=", 0, 0, 0),
//             Token::new("NUMBER", "10", 0, 0, 0),
//             Token::new("RIGHT_PARENTHESIS", ")", 0, 0, 0),
//             Token::new("LEFT_CURLY_BRACE", "{", 0, 0, 0),
//             Token::new("VARIABLE", "x", 0, 0, 0),
//             Token::new("EQUAL", "=", 0, 0, 0),
//             Token::new("NUMBER", "10", 0, 0, 0),
//             Token::new("SEMICOLON", ";", 0, 0, 0),
//             Token::new("PRINT", "print", 0, 0, 0),
//             Token::new("LEFT_PARENTHESIS", "(", 0, 0, 0),
//             Token::new("VARIABLE", "x", 0, 0, 0),
//             Token::new("RIGHT_PARENTHESIS", ")", 0, 0, 0),
//             Token::new("SEMICOLON", ";", 0, 0, 0),
//             Token::new("RIGHT_CURLY_BRACE", "}", 0, 0, 0),
//             Token::new("RIGHT_CURLY_BRACE", "}", 0, 0, 0),
//         ];
//         let tokens_actual = tokenize_code(&code_text);
//         for (expected, actual) in tokens_expected.iter().zip(tokens_actual.iter()) {
//             assert_eq!(expected, actual);
//         }
//     }
// }
