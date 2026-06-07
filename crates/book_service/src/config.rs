use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct DbConf {
    #[envconfig(from = "DB_PROTOCOL", default = "postgres")]
    pub protocol: String,
    #[envconfig(from = "DB_HOST")]
    pub host: String,
    #[envconfig(from = "DB_PORT", default = "5432")]
    pub port: u16,
    #[envconfig(from = "DB_USER")]
    pub user: String,
    #[envconfig(from = "DB_PASS")]
    pub password: String,
    #[envconfig(from = "DB_NAME")]
    pub db_name: String,
}

impl DbConf {
    pub fn init() -> Self {
        Self::init_from_env().expect("Failed to load configuration! Check the .env file.")
    }

    pub fn to_database_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.protocol, self.user, self.password, self.host, self.port, self.db_name
        )
    }
}
