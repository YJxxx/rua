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
) -> MaineServiceServer<api::Maine> {
    MaineServiceServer::with_interceptor(
        api::Maine::builder(model::Maine::builder(db), nc),
        intercept,
    )
}

fn intercept(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    let mut req = req;
    // req.metadata_mut()
    //     .insert("uid", "296347955659538975".parse().unwrap());
    Ok(req)
}
