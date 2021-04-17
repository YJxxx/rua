use actix::prelude::*;
use uuid::Uuid;

use crate::{pb::maine::maine_service_client::MaineServiceClient, rpc};

use std::collections::{HashMap, HashSet};

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: i64,
    pub msg: String,
    pub room: String,
}

pub struct ChatServer {
    pub id: String,
    pub sessions: HashMap<i64, Recipient<crate::message::Message>>,
    pub rooms: HashMap<String, HashSet<i64>>,
    pub nats: nats::Connection,
    pub rpc: rpc::Client,
}

impl ChatServer {
    pub fn new(nats: nats::Connection, rpc: rpc::Client) -> ChatServer {
        ChatServer {
            id: Uuid::new_v4().to_string(),
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            nats,
            rpc,
        }
    }
}

impl ChatServer {
    pub fn send_message(&self, room: &str, message: &str, skip_id: i64) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        // let _ = addr.do_send(crate::message::Message(message.to_owned()));
                    }
                }
            }
        }
    }
    pub fn push_message(&self, msg: impl AsRef<[u8]>) -> std::io::Result<()> {
        self.nats.publish("message", msg)
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {}
    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}
