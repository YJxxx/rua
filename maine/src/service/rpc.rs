use crate::pbteddy::teddy_service_client::TeddyServiceClient;
use dashmap::DashMap;
use envconfig::Envconfig;
use etcd::{Event, KeyRange};
use etcd_rs as etcd;
use futures::StreamExt;
use std::{collections::HashMap, time::Duration};
use tonic::transport::{Channel, Endpoint};
use tower::discover::Change;

#[derive(Debug, Clone)]
pub struct Client {
    pub teddys: DashMap<String, TeddyServiceClient<tonic::transport::Channel>>,
}

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "RPC_DST_TEDDY")]
    pub dst_teddy: String,
}

impl Client {
    pub async fn new(cfg: &Config) -> Self {
        let c = Client {
            teddys: DashMap::new(),
        };
        c.watch().await;
        c
    }

    async fn connect(&self, id: &str, dst: &str) {
        self.teddys.insert(
            id.into(),
            TeddyServiceClient::connect(dst.to_string().clone())
                .await
                .unwrap(),
        );
    }

    async fn watch(&self) {
        // todo:

        // let etcd = etcd::Client::connect(etcd::ClientConfig {
        //     endpoints: ["localhost:2379".into()].into(),
        //     auth: None,
        //     tls: None,
        // })
        // .await
        // .unwrap();

        // let mut inbound = etcd
        //     .watch(KeyRange::key("niceup.cn/rua/teddy"))
        //     .await
        //     .unwrap();
        // let s = self.clone();
    }
}
