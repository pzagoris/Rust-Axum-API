mod db;
mod rest;
use std::net::SocketAddr;
use crate::db::init_db;

// Using anyhow library
//Any-error bubbles to the top
use anyhow::Result;
use axum::{Extension, Router};

fn router(conn_pool: sqlx::SqlitePool) -> Router{
    Router::new()
        .nest_service("/students", rest::students_service())
        // Add the connection pool as a "layer", available for dependency injection.
        .layer(Extension(conn_pool))
}

#[tokio::main]
async fn main() -> Result<()> {

    // Load enviroment variables from .env if available
    dotenv::dotenv().ok();

    // Initialize the database and obtrain a connection pool
    let connection_pool = init_db().await?;

    // Initialize the Axum routing service
    let app = router(connection_pool);

    // Define the address to listen on 
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));

    // Start the server
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
