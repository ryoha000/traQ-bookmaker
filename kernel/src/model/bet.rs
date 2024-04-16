use derive_new::new;

use super::{candidate::Candidate, r#match::Match, user::User, Id};

#[derive(new, Debug)]
pub struct Bet {
    pub id: Id<Bet>,
    pub user_id: Id<User>,
    pub match_id: Id<Match>,
    pub candidate_id: Id<Candidate>,
    pub amount: i32,
}

#[derive(new, Debug)]
pub struct NewBetForLatestMatch {
    pub id: Id<Bet>,
    pub traq_id: String,
    pub channel_id: String,
    pub candidate_name: String,
    pub amount: i32,
}
