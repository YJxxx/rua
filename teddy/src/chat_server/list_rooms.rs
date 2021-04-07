use crate::server::ChatServer;
use actix::prelude::*;
use actix::Handler;

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();
        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }
        MessageResult(rooms)
    }
}
