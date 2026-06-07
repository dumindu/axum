use book_service::config::DbConf;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = DbConf::init().to_database_url();
    let config = Config::load()?;
    let db = toasty::Db::builder()
        .models(toasty::models!(book_service::models::Book, book_service::models::Author,))
        .connect(db_url.as_str())
        .await?;

    let cli = ToastyCli::with_config(db, config);

    cli.parse_and_run().await?;

    Ok(())
}
