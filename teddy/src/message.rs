use actix::Handler;

pub type Message = crate::pb::teddy::Message;
impl actix::Message for Message {
    type Result = ();
}

impl Handler<Message> for super::session::WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.binary(msg.body);
    }
}
