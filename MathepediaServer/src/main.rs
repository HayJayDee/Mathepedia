extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use crate::models::Theorem;

use self::schema::theorems::dsl::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let connection = &mut establish_connection();
    let results = theorems
        .limit(5)
        .select(Theorem::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
