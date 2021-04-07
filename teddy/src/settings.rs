use lazy_static::lazy_static;

use envconfig::Envconfig;

#[derive(Debug)]
pub struct Settings {
    pub rpc: crate::rpc::Config,
    pub nats: Nats,
    pub service: Service,
}

#[derive(Debug, Clone, Envconfig)]
pub struct Service {
    #[envconfig(from = "SERVICE_ADDR")]
    pub addr: String,
}

#[derive(Debug, Clone, Envconfig)]
pub struct Nats {
    #[envconfig(from = "NATS_URL")]
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
            nats: Nats::init_from_env().unwrap(),
            service: Service::init_from_env().unwrap(),
        }
    }

    pub fn get() -> &'static Self {
        &SETTINGS
    }
}
