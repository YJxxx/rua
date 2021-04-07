use crate::{
    chat_server::{ChatServer, MessagePayload},
    pb::teddy::{
        teddy_service_server::{TeddyService, TeddyServiceServer},
        SendMessageRep, SendMessageReq,
    },
};
use actix::Addr;
use tonic::{Request, Response, Status};
pub struct Teddy {
    server: Addr<ChatServer>,
}

impl Teddy {
    pub fn new_srv(server: Addr<ChatServer>) -> TeddyServiceServer<Teddy> {
        TeddyServiceServer::new(Teddy { server })
    }
}

#[tonic::async_trait]
impl TeddyService for Teddy {
    async fn send_message(
        &self,
        req: Request<SendMessageReq>,
    ) -> Result<Response<SendMessageRep>, Status> {
        // todo:

        // self.server
        //     .do_send(req.into_inner().message.unwrap() as PushMessage);
        Err(Status::internal("internal"))
    }
}
