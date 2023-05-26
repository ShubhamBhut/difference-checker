use axum::{
    handler::post,
    http::StatusCode,
    response::IntoResponse,
    AddExtensionLayer, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize)]
struct Difference {
    line: usize,
    character: usize,
    letter: char,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiffRequest {
    string1: String,
    string2: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiffResponse {
    differences: Vec<Difference>,
}

async fn diff(Json(request): Json<DiffRequest>) -> impl IntoResponse {
    let string1 = request.string1;
    let string2 = request.string2;

    let differences = diff_strings(&string1, &string2);

    let response = DiffResponse { differences };

    (StatusCode::OK, Json(response))
}

fn diff_strings(string1: &str, string2: &str) -> Vec<Difference> {
    let mut diffs = Vec::new();
    let chars1: Vec<char> = string1.chars().collect();
    let chars2: Vec<char> = string2.chars().collect();
    let mut line = 0;
    let mut character = 0;

    for (i, (char1, char2)) in chars1.iter().zip(chars2.iter()).enumerate() {
        if char1 != char2 {
            diffs.push(Difference {
                line,
                character,
                letter: *char2,
            });
        }
        if *char1 == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
    }

    diffs
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/diff", post(diff))
        .layer(AddExtensionLayer::new(diff_strings));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

