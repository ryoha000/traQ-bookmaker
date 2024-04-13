use derive_new::new;

use super::{channel::Channel, message::Message, Id};

#[derive(new, Debug)]
pub struct Match {
    pub id: Id<Match>,
    pub title: String,
    pub channel_id: Id<Channel>,
    pub message_id: Option<Id<Message>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deadline_at: chrono::DateTime<chrono::Utc>,
    pub status: MatchStatus,
}

#[derive(Debug)]
pub enum MatchStatus {
    Scheduled,
    OnGoing,
    Finished,
    Cancelled,
}

#[derive(new, Debug)]
pub struct NewMatch {
    pub id: Id<Match>,
    pub title: String,
    pub channel_id: Id<Channel>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deadline_at: chrono::DateTime<chrono::Utc>,
    pub status: MatchStatus,
}
