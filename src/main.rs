use axum::{
    extract::Json,
    handler::{get, post},
    response::Html,
    Router,
};
use serde::Deserialize;
use std::{convert::Infallible, net::SocketAddr};

#[derive(Deserialize)]
struct DiffRequest {
    string1: String,
    string2: String,
}

pub fn diff_strings_char(string1: &str, string2: &str) -> Vec<(usize, usize, char)> {
    let mut diffs = Vec::new();
    let lines1: Vec<&str> = string1.lines().collect();
    let lines2: Vec<&str> = string2.lines().collect();
    let mut line = 0;
    let mut character = 0;

    for (i, (line1, line2)) in lines1.iter().zip(lines2.iter()).enumerate() {
        let chars1: Vec<char> = line1.chars().collect();
        let chars2: Vec<char> = line2.chars().collect();
        let min_len = chars1.len().min(chars2.len());

        for j in 0..min_len {
            if chars1[j] != chars2[j] {
                diffs.push((line, character, chars2[j]));
            }
            character += 1;
        }

        if chars1.len() != chars2.len() {
            let max_len = chars1.len().max(chars2.len());
            let remaining_chars = if chars1.len() > chars2.len() {
                &chars1[min_len..max_len]
            } else {
                &chars2[min_len..max_len]
            };

            for ch in remaining_chars {
                diffs.push((line, character, *ch));
                character += 1;
            }
        }

        line += 1;
        character = 0;
    }

    diffs // Placeholder return value for demonstration purposes
}

async fn diff_handler(body: Json<DiffRequest>) -> Json<Vec<(usize, usize, char)>> {
    let DiffRequest { string1, string2 } = body.0;
    let diffs = diff_strings_char(&string1, &string2);
    Json(diffs)
}

async fn index_handler() -> Result<Html<&'static str>, Infallible> {
    Ok(Html(include_str!("static/index.html")))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/diff", post(diff_handler))
        .layer(axum::AddExtensionLayer::new("static/"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
