use axum::{
    Router, 
    routing::get
};

use std::fs;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_reason));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_reason() -> String {
    let content = fs::read_to_string("./reasons.txt").unwrap_or_else(|_| "No reason found.".to_string());
        
    let reasons = content.split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    if reasons.is_empty(){
        "No reason found.".to_string()
    } else {
        let random_index = rand::random_range(0..reasons.len());
        reasons[random_index].to_string()
    }
}
