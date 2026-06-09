use toasty::Db;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}
