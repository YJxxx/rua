use crate::pb::{
    maine::maine_service_client::MaineServiceClient,
    ragdoll::ragdoll_internal_service_client::RagdollInternalServiceClient,
};

#[derive(Debug, Clone)]
pub struct Client {
    pub regdoll: RagdollInternalServiceClient<tonic::transport::Channel>,
    pub maine: MaineServiceClient<tonic::transport::Channel>,
}

use envconfig::Envconfig;

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "RPC_DST_RAGDOLL")]
    pub dst_ragdoll: String,
    #[envconfig(from = "RPC_DST_MAINE")]
    pub dst_maine: String,
}

impl Client {
    pub async fn new(cfg: &Config) -> Self {
        let cfg = cfg.clone();
        Client {
            regdoll: RagdollInternalServiceClient::<tonic::transport::Channel>::connect(
                cfg.dst_ragdoll,
            )
            .await
            .unwrap(),
            maine: MaineServiceClient::<tonic::transport::Channel>::connect(cfg.dst_maine)
                .await
                .unwrap(),
        }
    }
}
