#[macro_use]
extern crate lazy_static;
extern crate thiserror;

use controller::Api;
use poem::{
    listener::TcpListener, middleware::Cors, EndpointExt,
    Result, Route, Server,
};
use poem_openapi::{
    OpenApiService,
};

pub mod controller;
// pub mod dao;
pub mod domain;
pub mod service;
pub mod utils;
type DbPool = sqlx::SqlitePool;#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    // env_logger::init();
    tracing_subscriber::fmt::init();
    let pool = DbPool::connect("sqlite://firm.db").await?;
    let api_service =
        OpenApiService::new(Api, "Firm Api", "1.0.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(pool);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(route)
        .await?;
    Ok(())
}
