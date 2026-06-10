use toasty::Db;

use crate::ServerConf;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub server_conf: ServerConf,
}
