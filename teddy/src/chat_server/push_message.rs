use actix::prelude::*;
use actix::Handler;
use bytes::Bytes;

use super::ChatServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessagePayload {
    header: MessagePayloadHeader,
    body: Bytes,
}

impl MessagePayload {
    pub fn new(header: MessagePayloadHeader, body: Bytes) -> Self {
        MessagePayload { header, body }
    }
}

pub struct MessagePayloadHeader {
    id: i64,
}

impl MessagePayloadHeader {
    pub fn new(id: i64) -> Self {
        MessagePayloadHeader { id }
    }
}

impl Handler<MessagePayload> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: MessagePayload, _: &mut Context<Self>) {
        self.push_message(msg.body).unwrap();
    }
}
