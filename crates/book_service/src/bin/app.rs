use book_service::{AppConf, AppState, routes};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json().with_target(false))
        .init();

    let conf = AppConf::init();

    let db = toasty::Db::builder()
        .models(toasty::models!(book_service::models::Book, book_service::models::Author))
        .connect(&conf.db.to_database_url())
        .await?;
    let state = AppState { db };

    let addr = conf.sever.to_addr();
    info!(addr = %addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let app = routes::init(state);
    axum::serve(listener, app).await?;

    Ok(())
}
