use dotenvy::dotenv;
use std::net::SocketAddr;

mod config;
mod db;
mod modules;
mod routes;
mod state;
mod utils;

use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let state = AppState {};

    let app = routes::routes::get_routes()
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server started at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}