use crate::card::Hand;
use crate::message::Message;
use actix::{dev::MessageResponse, Actor, ActorContext, Context, Handler, StreamHandler};
use actix_web_actors::ws::WebsocketContext;
pub struct User {
    pub hand: Hand,
}

impl Actor for User {
    type Context = Context<Self>;
}

impl Handler<Message> for User {
    type Result = Result<(), String>;
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Message::OutUser(deck, set) => {
                if set.is_greater_than(&deck) {
                    if !self.hand.subtract(&set) {
                        return Err("cards is not enough".into());
                    }
                    return Ok(());
                }
                Err("cards is not greater than deck".into())
            }
            _ => Err("unsupported message".into()),
        }
    }
}
