use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use serde::{Deserialize, Serialize};

mod api; 
mod models;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub api_version: u8,
    pub data: Option<PlayerCountResponse>,
}

#[derive(Serialize)]
pub struct PlayerCountResponse {
    pub players: u32,
    pub max_players: u32,
    pub avg_players: u32,
}

#[get("/")]
async fn hello() -> impl Responder {
    for i in 0..10 {
        println!("Item {}!", i);
    }

    HttpResponse::Ok().body("Welcome to the Bloody API V2 built with WASM!")
}

#[get("/player_count")]
async fn player_count() -> impl Responder {
    println!("Player count requested!");

    let response_json = &GenericResponse {
        status: "success".to_string(),
        api_version: 2,
        data: Some(PlayerCountResponse {
            players: 541,
            max_players: 1900,
            avg_players: 0,
        }),
    };

    HttpResponse::Ok().json(response_json)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(player_count)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}