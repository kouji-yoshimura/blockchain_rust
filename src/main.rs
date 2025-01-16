use axum::{
    routing::{get, post},
    response::IntoResponse,
    extract::State,
    Router,
    Json,
};
use serde::Deserialize;
use dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;
use blockchain_rust::{
    block::Block,
    database::{
        Database,
        sqlite::SqliteDatabase,
    },
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    let db = SqliteDatabase::new(pool);

    let app = Router::new()
        .route("/blocks", get(blocks::<SqliteDatabase>))
        .route("/mine_block", post(mine_block::<SqliteDatabase>))
        .route("/peers", get(peers))
        .route("/add_peers", post(add_peers))
        .route("/send_transaction", post(send_transaction))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn blocks<T: Database>(State(db): State<T>) -> impl IntoResponse {
    let block_chain = db.find_block_chain().await.unwrap();
    Json(block_chain)
}

#[derive(Debug, Deserialize)]
struct MineBlockRequest {
    data: String,
}

async fn mine_block<T: Database>(
    State(db): State<T>,
    Json(request): Json<MineBlockRequest>
) -> impl IntoResponse {
    let mut block_chain = db.find_block_chain().await.unwrap();
    let new_block = block_chain.append_new_block(vec!());

    db.save_block(new_block.clone()).await.unwrap();
    Json(new_block)
}

async fn peers() {}

async fn add_peers() {}

async fn send_transaction() {

}

