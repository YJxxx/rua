use std::time::Duration;

use crate::pbteddy;
use crate::service::rpc;
use etcd_rs::KeyRange;
use futures::StreamExt;
use prost::Message;
pub struct Client {
    nc: nats::Connection,
    rpc: rpc::Client,
}

impl Client {
    pub fn new(nc: nats::Connection, rpc: rpc::Client) -> Self {
        Client { nc, rpc }
    }

    pub async fn start(&self) {
        // let sub = self.nc.subscribe("messages").unwrap();
        // tokio::spawn(async move {
        //     loop {
        //         match sub.next() {
        //             Some(msg) => {
        //                 // insert db
        //                 let msg = pbteddy::Message::decode(msg.data.as_ref()).unwrap();
        //                 if let Some(client) = self.rpc.teddys.get("") {
        //                     let mut client = client.clone();
        //                     match client
        //                         .send_message(pbteddy::SendMessageReq { message: Some(msg) })
        //                         .await
        //                     {
        //                         Ok(_) => {}
        //                         Err(_) => {}
        //                     }
        //                 };
        //             }
        //             None => {
        //                 break;
        //             }
        //         }
        //     }
        // });
    }
}
