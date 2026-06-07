use book_service::{AppConf, AppState, routes};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let conf = AppConf::init();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json().with_target(false))
        .init();

    let state = AppState {};
    let app = routes::init(state);
    let listener = tokio::net::TcpListener::bind(conf.sever.to_addr()).await.unwrap();

    info!(addr = conf.sever.to_addr(), "Starting server");

    axum::serve(listener, app).await.unwrap();
}
