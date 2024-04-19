use derive_new::new;

use super::{message::Message, Id};

#[derive(Debug)]
pub enum StampType {
    WhiteCheckMark,
}

#[derive(new, Debug)]
pub struct NewStamp {
    pub message_id: Id<Message>,
    pub stamp_type: StampType,
}
