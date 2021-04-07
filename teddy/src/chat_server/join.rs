use crate::server::ChatServer;
use actix::prelude::*;
use actix::Handler;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: i64,
    pub name: String,
}

impl Handler<Join> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
        self.rooms
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);

        self.send_message(&name, "Someone connected", id);
    }
}
