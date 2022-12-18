use crate::{
    card::Hand,
    message::{InnerMessage, Response},
};
use actix::{Actor, Addr, Context, Handler};
use actix_web_actors::ws;

use crate::user::User;

#[derive(Clone)]
pub struct Desktop {
    users: Vec<Addr<User>>,
}

impl Desktop {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
}

impl Actor for Desktop {
    type Context = Context<Self>;
}

impl Handler<InnerMessage> for Desktop {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: InnerMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InnerMessage::Sit { uid, desktop_id, ws_addr } => {
                if self.users.len() == 6 {
                    ws_addr
                        .try_send(Response::Sit {
                            is_ok: false,
                            reason: "desktop is full".into(),
                        })
                        .unwrap();
                    return Ok(());
                }
            }
            _ => {}
        }
        Ok(())
    }
}
