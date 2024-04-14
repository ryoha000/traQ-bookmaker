use derive_new::new;
use kernel::model::{
    r#match::{MatchStatus, NewMatch},
    Id,
};

#[derive(new)]
pub struct CreateMatch {
    pub title: String,
    pub channel_id: String,
}

impl From<CreateMatch> for NewMatch {
    fn from(c: CreateMatch) -> Self {
        NewMatch::new(
            Id::gen(),
            c.title,
            Id::new(c.channel_id),
            chrono::Utc::now(),
            MatchStatus::Scheduled,
        )
    }
}
