use derive_new::new;

use super::{channel::Channel, Id};

#[derive(new, Debug)]
pub struct Message {
    pub id: Id<Message>,
    pub channel_id: Id<Channel>,
}

#[derive(new, Debug)]
pub struct NewMessage {
    pub channel_id: Id<Channel>,
    pub content: String,
    pub embed: bool,
}
