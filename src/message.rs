use crate::{
    card::{Card, CardSet},
    WS,
};
use actix::{self, Addr};
use serde::{Deserialize, Serialize};

impl actix::Message for Message {
    type Result = Result<(), String>;
}

#[derive(Deserialize)]
pub enum Message {
    Sit { uid: String, desktop_id: usize },
    Join,
    Skip,
}

impl actix::Message for InnerMessage {
    type Result = Result<(), String>;
}

pub enum InnerMessage {
    Sit { uid: String, desktop_id: usize, ws_addr: Addr<WS> },
    Out { deck: CardSet, set: CardSet },
}

impl actix::Message for Response {
    type Result = Result<(), String>;
}

#[derive(Serialize)]
pub enum Response {
    Sit { is_ok: bool, reason: String },
}
