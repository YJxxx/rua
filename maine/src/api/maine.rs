use crate::pb::{self, maine_service_server::MaineService};
use crate::{model, service};

use redis::Value as RV;
use redis::{aio::MultiplexedConnection, AsyncCommands};
use tonic::{Request, Response, Status};
pub struct Maine {
    model: model::Maine,
    nc: nats::Connection,
    redis: redis::Client,
}

impl Maine {
    pub fn builder(model: model::Maine, nc: nats::Connection, redis: redis::Client) -> Self {
        Maine { model, nc, redis }
    }
}

static ONLINE: &'static str = "online";

#[tonic::async_trait]
impl MaineService for Maine {
    async fn online(
        &self,
        request: Request<pb::OnlineReq>,
    ) -> Result<Response<pb::OnlineRep>, Status> {
        let request = request.get_ref();
        return match self.redis.get_async_connection().await {
            Ok(mut con) => {
                match con
                    .hset::<&str, u64, String, RV>(ONLINE, request.id, request.service_id.clone())
                    .await
                {
                    Ok(_) => Ok(Response::new(pb::OnlineRep {})),
                    Err(err) => Err(tonic::Status::internal("")),
                }
            }
            Err(_) => Err(tonic::Status::internal("")),
        };
    }

    async fn offline(
        &self,
        request: Request<pb::OfflineReq>,
    ) -> Result<Response<pb::OfflineRep>, Status> {
        let request = request.get_ref();
        return match self.redis.get_async_connection().await {
            Ok(mut con) => match con.hdel::<&str, u64, RV>(ONLINE, request.id).await {
                Ok(_) => Ok(Response::new(pb::OfflineRep {})),
                Err(err) => Err(tonic::Status::internal("")),
            },
            Err(_) => Err(tonic::Status::internal("")),
        };
    }
}
