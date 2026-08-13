use book_service::{AppConf, AppState, routes};
use mimalloc::MiMalloc;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "info,tower_http=warn,axum=warn,toasty=warn,tokio_postgres=warn".into()
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json().with_target(false).with_current_span(false))
        .init();

    let conf = AppConf::init();

    let db = toasty::Db::builder()
        .models(toasty::models!(book_service::models::Book, book_service::models::Author))
        .connect(&conf.db.to_database_url())
        .await?;

    let addr = conf.server.to_addr();
    let server_conf = conf.server;
    let state = AppState { db, server_conf };

    info!(addr = %addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let app = routes::init(state);
    axum::serve(listener, app).await?;

    Ok(())
}
