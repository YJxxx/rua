mod api;
mod consumer;
mod model;
mod schema;
mod service;
mod settings;
mod srv;
mod statusx;
mod pb {
    tonic::include_proto!("rua.maine.v1");
}
mod pbteddy {
    tonic::include_proto!("rua.teddy.v1");
}
mod pbmessage {
    tonic::include_proto!("rua.message.v1");
}

use consumer::Client;
use service::{db, rpc};
use tokio::{signal, sync::oneshot};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = crate::settings::Settings::get();

    let db = db::DB::new(&cfg.db).await;

    let rpc = rpc::Client::new(&cfg.rpc).await;

    let nc = nats::connect(cfg.nats.url.as_str()).unwrap();

    let consumer = Client::new(nc.clone(), rpc.clone());

    let (tx, rx) = oneshot::channel();
    tokio::spawn(async {
        let _ = signal::ctrl_c().await;
        tx.send(())
    });
    consumer.start().await;
    Server::builder()
        .add_service(srv::new_srv(db, rpc, nc).await)
        .serve_with_shutdown(cfg.service.addr.parse().unwrap(), async {
            rx.await.ok();
        })
        .await?;
    Ok(())
}
