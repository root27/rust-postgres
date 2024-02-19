use actix_web::{web,App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod models;
mod handlers;

use handlers::*;



pub struct AppState {
    db: Pool<Postgres>
}




#[actix_web::main]
async fn main() -> std::io::Result<()>  {


    let url = "postgres://test:123@127.0.0.1:5432/rust_db";


    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(url).await {
            Ok(pool) => {
                println!("Connected to DB");
                pool
            }
            Err(e) => {
                panic!("Failed to connect to Postgres: {:?}", e);
            }
        };
    

    println!("Server is running at port 8080");



    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { db: pool.clone() }))
        .route("/create", web::post().to(create))
        .route("/get_all", web::get().to(get_all))
        .route("/get/{id}", web::get().to(get))
        .route("/update/{id}", web::put().to(update))
        .route("/delete/{id}", web::delete().to(delete))
        
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await



}
