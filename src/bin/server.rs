use sample_rust_api::config;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    use config::db::DbPool;

    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let pg_pool = sqlx::PgPool::retrieve().await;
    let addr = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::debug!("listening on 127.0.0.1:8080 !");
    let server = 
        axum::serve(addr, sample_rust_api::app(pg_pool).into_make_service());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
