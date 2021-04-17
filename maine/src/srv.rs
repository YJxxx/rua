use crate::{
    api, model,
    pb::maine_service_server::MaineServiceServer,
    service::{db, rpc},
};

use envconfig::Envconfig;

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "SERVICE_ADDR")]
    pub addr: String,
}

pub async fn new_srv(
    db: db::DB,
    rpc: rpc::Client,
    nc: nats::Connection,
    redis: redis::Client,
) -> MaineServiceServer<api::Maine> {
    MaineServiceServer::new(api::Maine::builder(model::Maine::builder(db), nc, redis))
}
