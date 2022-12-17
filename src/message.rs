use crate::card::{Card, CardSet};
use actix;

impl actix::Message for Message {
    type Result = Result<(), String>;
}
pub enum Message {
    Join,
    Out(Vec<Card>),
    OutUser(CardSet, CardSet),
    Skip,
}
