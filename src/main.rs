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

#[derive(Deserialize)]
struct TokenRequest {
    code_text: String,
    mode: String,
}

async fn generate_tokens(Json(body): Json<TokenRequest>) -> impl IntoResponse {
    let start_time = Instant::now();
    let tokens = lexer::tokenize_code(body.code_text.as_str());
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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/tokens", post(generate_tokens))
        .route("/tab1", get(tab1))
        .route("/tab2", get(tab2));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on -> http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::lexer::{tokenize_code, Token};
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn test_tokenize_code() {
        let file_path = Path::new("example_code.txt");
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(err) => panic!("Error opening file: {:?}", err),
        };
        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => {
                let code_text = buffer.to_string();
                code_text
            }
            Err(err) => panic!("Error reading file: {:?}", err),
        };
        let code_text = buffer.as_str();
        let tokens_expected: Vec<Token> = vec![
            Token::new("identifier", "program", 0, 0, 0),
            Token::new("left_parenthesis", "(", 0, 0, 0),
            Token::new("right_parenthesis", ")", 0, 0, 0),
            Token::new("left_curly_brace", "{", 0, 0, 0),
            Token::new("keyword_while_e", "while", 0, 0, 0),
            Token::new("left_parenthesis", "(", 0, 0, 0),
            Token::new("identifier", "x", 0, 0, 0),
            Token::new("less_than_or_equal_operator", "<=", 0, 0, 0),
            Token::new("numeric_literal", "10", 0, 0, 0),
            Token::new("right_parenthesis", ")", 0, 0, 0),
            Token::new("left_curly_brace", "{", 0, 0, 0),
            Token::new("identifier", "x", 0, 0, 0),
            Token::new("assignment_operator", "=", 0, 0, 0),
            Token::new("numeric_literal", "10", 0, 0, 0),
            Token::new("semicolon", ";", 0, 0, 0),
            Token::new("keyword_print_t", "print", 0, 0, 0),
            Token::new("left_parenthesis", "(", 0, 0, 0),
            Token::new("identifier", "x", 0, 0, 0),
            Token::new("right_parenthesis", ")", 0, 0, 0),
            Token::new("semicolon", ";", 0, 0, 0),
            Token::new("right_curly_brace", "}", 0, 0, 0),
            Token::new("right_curly_brace", "}", 0, 0, 0),
        ];
        let tokens_actual = tokenize_code(&code_text);
        for (expected, actual) in tokens_expected.iter().zip(tokens_actual.iter()) {
            assert_eq!(expected, actual);
        }
    }
}
