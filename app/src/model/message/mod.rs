pub mod help;
pub mod r#match;

use derive_new::new;
use kernel::model::{message::NewMessage, Id};

#[derive(new)]
pub struct SendMessage {
    pub channel_id: String,
    pub content: String,
    pub embed: bool,
}

impl From<SendMessage> for NewMessage {
    fn from(c: SendMessage) -> Self {
        NewMessage::new(Id::new(c.channel_id), c.content, c.embed)
    }
}
