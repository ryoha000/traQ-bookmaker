use std::collections::HashMap;

use super::{bet::Bet, candidate::Candidate};

#[derive(Debug)]
pub struct Statistic {
    pub candidate: Candidate,
    pub amount: i32, // DraftStatistic.bets.sum(bet.amount)
    pub rate: f64,   // Vec<Statistic>.sum(statistic.amount) / statistic.amount
    pub bets: Vec<Bet>,
}

struct DraftStatistic {
    pub candidate: Candidate,
    pub bets: Vec<Bet>,
}

pub fn new_statistics(bets: Vec<Bet>, candidates: Vec<Candidate>) -> Vec<Statistic> {
    let mut statistics = Vec::new();
    let mut draft_statistics = candidates
        .into_iter()
        .map(|c| {
            (
                c.id.value.clone(),
                DraftStatistic {
                    candidate: c,
                    bets: Vec::new(),
                },
            )
        })
        .collect::<HashMap<_, _>>();
    for bet in bets {
        let candidate_id = bet.candidate_id.value.clone();
        if let Some(draft_statistic) = draft_statistics.get_mut(&candidate_id) {
            draft_statistic.bets.push(bet);
        }
    }
    for (_, draft_statistic) in draft_statistics {
        let amount = draft_statistic.bets.iter().map(|b| b.amount).sum();
        let rate = amount as f64 / draft_statistic.bets.len() as f64;
        statistics.push(Statistic {
            candidate: draft_statistic.candidate,
            amount,
            rate,
            bets: draft_statistic.bets,
        });
    }
    statistics.sort_by(|a, b| b.amount.cmp(&a.amount));
    statistics
}
