use super::ChatServer;
use actix::prelude::*;
use actix::Handler;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i64,
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let mut rooms: Vec<String> = Vec::new();
        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}
