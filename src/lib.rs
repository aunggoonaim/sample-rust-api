#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern  crate clap;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod dto;
mod error;
mod extractors;
mod handlers;
mod model;
mod service;
mod sql;
mod utils;

pub mod config;

pub fn app(pg_pool: PgPool) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(pg_pool))
        .into_inner();

    Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register))
        .route("/authorize", get(handlers::authorize))
        .layer(middleware_stack)
}
