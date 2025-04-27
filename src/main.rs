pub mod api;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use api::{auth, db::connect_db, log::{log, LogType}};
use axum::{routing::post, Router};


#[tokio::main]
async fn main() {
    
    log(LogType::SETUP, "Setting up PenSpecter server...");
    let db = connect_db().await;

    log(LogType::SETUP, "Creating routing listeners...");
    let http_server: Router = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .with_state(db.clone());

    log(LogType::SETUP, "Server listening on port 8080.");
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, http_server).await.expect("Failed to start backend server!");

}
