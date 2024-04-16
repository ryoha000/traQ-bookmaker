use derive_new::new;
use kernel::model::{
    r#match::{NewMatch, UpdateMatchForLatest},
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
        )
    }
}

#[derive(new)]
pub struct CloseMatch {
    pub channel_id: String,
}

impl From<CloseMatch> for UpdateMatchForLatest {
    fn from(c: CloseMatch) -> Self {
        UpdateMatchForLatest::new(Id::new(c.channel_id), Some(Some(chrono::Utc::now())), None)
    }
}
