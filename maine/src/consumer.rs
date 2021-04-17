use std::time::Duration;

use crate::pbteddy;
use crate::service::rpc;
use etcd_rs::KeyRange;
use futures::StreamExt;
use prost::Message;
use redis::Value as RV;
use redis::{aio::MultiplexedConnection, AsyncCommands};
pub struct Client {
    nc: nats::Connection,
    rpc: rpc::Client,
    redis: redis::Client,
}

impl Client {
    pub fn new(nc: nats::Connection, rpc: rpc::Client, redis: redis::Client) -> Self {
        Client { nc, rpc, redis }
    }

    pub async fn start(&self) {
        let sub = self.nc.subscribe("messages").unwrap();
        let rpc = self.rpc.clone();
        let redis = self.redis.clone();
        tokio::spawn(async move {
            loop {
                match sub.next() {
                    Some(msg) => {
                        let mut con = redis.get_async_connection().await.unwrap();
                        let msg = pbteddy::Message::decode(msg.data.as_ref()).unwrap();
                        match con
                            .hget::<&str, i64, RV>("online", msg.header.as_ref().unwrap().id)
                            .await
                        {
                            Ok(v) => match v {
                                RV::Bulk(value) => {
                                    if let Some(mut client) = rpc.teddys.get_mut("key") {
                                        if let Some(status) = client
                                            .send_message(pbteddy::SendMessageReq {
                                                message: Some(msg),
                                            })
                                            .await
                                            .err()
                                        {
                                            println!("{:?}", status);
                                        }
                                    };
                                }
                                _ => {}
                            },
                            Err(_) => {}
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        });
    }
}
