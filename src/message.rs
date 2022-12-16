use crate::card::Card;
pub enum Message {
    Join(i32),
    Out(Vec<Card>),
    Skip,
}
