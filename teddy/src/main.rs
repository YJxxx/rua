use actix::*;
use actix_web::{web, App, HttpServer};

mod api;
mod chat_server;
mod message;
mod route;
mod rpc;
mod session;
mod settings;
use crate::chat_server::ChatServer;

mod pb {
    pub mod teddy {
        tonic::include_proto!("rua.teddy.v1");
    }
    pub mod maine {
        tonic::include_proto!("rua.maine.v1");
    }
    pub mod ragdoll {
        tonic::include_proto!("rua.ragdoll.v1");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cfg = crate::settings::Settings::get();

    let nats = nats::connect(cfg.nats.url.as_str()).unwrap();

    let rpc = rpc::Client::new(&cfg.rpc).await;

    let server = ChatServer::new(nats.clone(), rpc).start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(web::resource("/ws/").to(route::chat_route))
    })
    .bind(cfg.service.addr.as_str())?
    .run()
    .await
}
