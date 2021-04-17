use lazy_static::lazy_static;

use envconfig::Envconfig;

#[derive(Debug)]
pub struct Settings {
    pub rpc: crate::rpc::Config,
    pub service: crate::srv::Config,
    pub db: crate::db::Config,
    pub nats: Nats,
    pub redis: Redis,
}

#[derive(Debug, Clone, Envconfig)]
pub struct Nats {
    #[envconfig(from = "NATS_URL")]
    pub url: String,
}

#[derive(Debug, Clone, Envconfig)]
pub struct Redis {
    #[envconfig(from = "REDIS_URL")]
    pub url: String,
}

lazy_static! {
    static ref SETTINGS: Settings = {
        dotenv::dotenv().ok();
        Settings::init()
    };
}

impl Settings {
    fn init() -> Self {
        Settings {
            rpc: crate::rpc::Config::init_from_env().unwrap(),
            service: crate::srv::Config::init_from_env().unwrap(),
            db: crate::db::Config::init_from_env().unwrap(),
            nats: Nats::init_from_env().unwrap(),
            redis: Redis::init_from_env().unwrap(),
        }
    }

    pub fn get() -> &'static Self {
        &SETTINGS
    }
}
