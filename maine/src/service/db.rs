extern crate envconfig;
extern crate envconfig_derive;

use envconfig::Envconfig;

use sqlx::mysql::MySqlPool;

#[derive(Debug, Envconfig)]
pub struct Config {
    #[envconfig(from = "MYSQL_DSN")]
    pub dsn: String,
}

#[derive(Clone)]
pub struct DB {
    pub pool: MySqlPool,
}

impl DB {
    pub async fn new(cfg: &Config) -> Self {
        DB {
            pool: MySqlPool::connect(&cfg.dsn).await.unwrap(),
        }
    }
}
