use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use urlencoding::encode;

#[derive(Deserialize)]
struct Query {
    q: String,
}

#[get("/api/news")]
async fn get_news(query: web::Query<Query>) -> impl Responder {
    let api_key = env::var("NEWS_API_KEY").expect("API key not found");
    
    let search_terms = if query.q.is_empty() {
        "cryptocurrency OR bitcoin OR ethereum".to_string()
    } else {
        format!("{} AND (cryptocurrency OR bitcoin OR ethereum)", query.q)
    };

    let url = format!(
        "https://newsapi.org/v2/everything?q={}&language=en&sortBy=publishedAt&apiKey={}",
        encode(&search_terms), api_key
    );

    // Create a client with custom User-Agent
    let client = reqwest::Client::builder()
        .user_agent("YourNewsApp/1.0")  // ← THIS IS THE CRITICAL FIX
        .build()
        .unwrap();

    match client.get(&url).send().await {
        Ok(res) => {
            if res.status().is_success() {
                match res.text().await {
                    Ok(body) => HttpResponse::Ok().content_type("application/json").body(body),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to read response"),
                }
            } else {
                let status = res.status();
                let body = res.text().await.unwrap_or_default();
                println!("News API error: {} - {}", status, body);
                HttpResponse::BadRequest().body(format!("News API error: {}", status))
            }
        }
        Err(e) => {
            println!("Request failed: {}", e);
            HttpResponse::InternalServerError().body(format!("Failed to fetch news: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init(); // Add this for better logging

    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(get_news)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}