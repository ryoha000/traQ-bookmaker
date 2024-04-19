use derive_new::new;
use kernel::model::{bet::NewBetForLatestMatch, Id};

#[derive(new)]
pub struct CreateBet {
    pub channel_id: String,
    pub traq_id: String,
    pub candidate_name: String,
    pub amount: i32,
}

impl From<CreateBet> for NewBetForLatestMatch {
    fn from(c: CreateBet) -> Self {
        NewBetForLatestMatch::new(
            Id::gen(),
            c.traq_id,
            c.channel_id,
            c.candidate_name,
            c.amount,
        )
    }
}
