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
    // Process the token request here (e.g., generate a token)
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
