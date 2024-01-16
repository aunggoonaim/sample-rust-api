#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use std::time::Duration;

use axum::{
    routing::{get, post},
    Router, error_handling::HandleErrorLayer, BoxError, http::StatusCode,
};
use sqlx::PgPool;
use tower::{ServiceBuilder, timeout::TimeoutLayer};
use tower_http::add_extension::AddExtensionLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod error;
mod middleware;
mod handlers;
mod controller;
mod service;
mod model;
mod entity;
mod utils;
mod query;

pub mod config;

pub fn app(pg_pool: PgPool) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(pg_pool))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
        )
        .into_inner();

    Router::new()
        .route("/", get(handlers::home))
        .nest("/user", controller::user_controller::user_router())
        .nest("/role", controller::role_controller::role_router())
        .layer(middleware_stack)
        .route_layer(axum::middleware::from_fn(middleware::auth))
}
