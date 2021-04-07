use crate::model;
use crate::pb::{self, maine_service_server::MaineService};

use redis::{aio::MultiplexedConnection, RedisResult};
use tonic::{Request, Response, Status};

use std::collections::{BTreeMap, BTreeSet};
use std::collections::{HashMap, HashSet};
use std::thread::{sleep, spawn};
use std::time::Duration;

pub struct Maine {
    model: model::Maine,
    nc: nats::Connection,
}

impl Maine {
    pub fn builder(model: model::Maine, nc: nats::Connection) -> Self {
        Maine { model, nc }
    }
}

#[tonic::async_trait]
impl MaineService for Maine {
    async fn online(
        &self,
        request: Request<pb::OnlineReq>,
    ) -> Result<Response<pb::OnlineRep>, Status> {
        todo!()
    }

    async fn offline(
        &self,
        request: Request<pb::OfflineReq>,
    ) -> Result<Response<pb::OfflineRep>, Status> {
        todo!()
    }
}
