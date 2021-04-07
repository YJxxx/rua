use super::ChatServer;
use actix::prelude::*;
use actix::Handler;

/// New chat session is created
#[derive(Message)]
#[rtype(i64)]
pub struct Connect {
    pub addr: Recipient<crate::message::Message>,
}

impl Handler<Connect> for ChatServer {
    type Result = i64;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = 1; // TODO:
        self.sessions.insert(id, msg.addr);
        id
    }
}
