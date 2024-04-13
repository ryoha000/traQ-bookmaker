pub mod help;

use derive_new::new;
use kernel::model::{message::NewMessage, Id};

#[derive(new)]
pub struct CreateMessage {
    pub channel_id: String,
    pub content: String,
    pub embed: bool,
}

impl From<CreateMessage> for NewMessage {
    fn from(c: CreateMessage) -> Self {
        NewMessage::new(Id::new(c.channel_id), c.content, c.embed)
    }
}
