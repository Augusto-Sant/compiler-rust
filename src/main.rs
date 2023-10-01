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
}

async fn generate_token(Json(body): Json<TokenRequest>) -> impl IntoResponse {
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

    Response::new(rendered).into_response()
}

async fn index() -> impl IntoResponse {
    let tera = Tera::new("templates/**/*").unwrap();
    let context = Context::new();
    let rendered = tera.render("index_template.html", &context).unwrap();
    Response::new(rendered)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/tokens", post(generate_token));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on -> {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::lexer;

    #[test]
    fn test_tokenize_code() {
        let code_text = r#"
            fn program(){
                while(x<=10){
                    x = 10;
                    print(x);
                }
                
            }
            "#;
        let mut tokens_expected: Vec<lexer::Token> = Vec::new();
        tokens_expected.push(lexer::Token::new("function_keyword_n", "fn", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("identifier", "program", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("left_parenthesis", "(", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("right_parenthesis", ")", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("left_curly_brace", "{", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("keyword_while_e", "while", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("left_parenthesis", "(", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("identifier", "x", 0, 0, 0));
        tokens_expected.push(lexer::Token::new(
            "less_than_or_equal_operator",
            "<=",
            0,
            0,
            0,
        ));
        tokens_expected.push(lexer::Token::new("numeric_literal", "10", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("right_parenthesis", ")", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("left_curly_brace", "{", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("identifier", "x", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("assignment_operator", "=", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("numeric_literal", "10", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("semicolon", ";", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("keyword_print_t", "print", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("left_parenthesis", "(", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("identifier", "x", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("right_parenthesis", ")", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("semicolon", ";", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("right_curly_brace", "}", 0, 0, 0));
        tokens_expected.push(lexer::Token::new("right_curly_brace", "}", 0, 0, 0));

        let tokens_actual = lexer::tokenize_code(code_text);
        for (expected, actual) in tokens_expected.iter().zip(tokens_actual.iter()) {
            assert_eq!(expected, actual);
        }
    }
}
