use derive_new::new;

use super::{candidate::Candidate, channel::Channel, message::Message, DateTimeUtc, Id};

#[derive(new, Debug)]
pub struct Match {
    pub id: Id<Match>,
    pub title: String,
    pub channel_id: Id<Channel>,
    pub message_id: Option<Id<Message>>,
    pub created_at: DateTimeUtc,
    pub closed_at: Option<DateTimeUtc>,
    pub winner_candidate_id: Option<Id<Candidate>>,
}

#[derive(new, Debug)]
pub struct NewMatch {
    pub id: Id<Match>,
    pub title: String,
    pub channel_id: Id<Channel>,
    pub created_at: DateTimeUtc,
}

#[derive(new, Debug)]
pub struct UpdateMatchForLatest {
    pub channel_id: Id<Channel>,
    pub closed_at: Option<Option<DateTimeUtc>>,
    pub winner_candidate_id: Option<Option<Id<Candidate>>>,
}
