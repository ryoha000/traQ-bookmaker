use derive_new::new;
use kernel::model::{channel::Channel, r#match::Match, Id};

#[derive(new, Debug)]
pub struct UpsertMatchMessage {
    pub channel_id: Id<Channel>,
    pub match_: Match,
}
