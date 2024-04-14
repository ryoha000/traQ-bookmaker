use derive_new::new;

use super::{r#match::Match, Id};

#[derive(new, Debug)]
pub struct Candidate {
    pub id: Id<Candidate>,
    pub name: String,
    pub match_id: Id<Match>,
    pub is_winner: Option<bool>,
}

#[derive(new, Debug)]
pub struct NewCandidate {
    pub id: Id<Candidate>,
    pub name: String,
    pub match_id: Id<Match>,
}
