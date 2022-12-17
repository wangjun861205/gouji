use crate::{card::Hand, message::Message};
use actix::{Actor, Addr, StreamHandler};

use crate::user::User;
use actix_web_actors::ws::{self, WebsocketContext};

pub struct Desktop {
    addrs: Vec<Addr<User>>,
}

impl Actor for Desktop {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Message> for Desktop {
    fn handle(&mut self, item: Message, ctx: &mut Self::Context) {
        match item {
            Join => {
                let user = User { hand: Hand::new(Vec::new()) };
                let addr = user.start();
                self.addrs.push(addr);
            }
        }
    }
}
