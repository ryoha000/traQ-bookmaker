use std::collections::HashMap;

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
pub struct Statistic {
    pub candidate_id: Id<Candidate>,
    pub amount: i32,
    pub rate: f64,
    pub bets: Vec<Bet>,
}

pub fn calculate_statistics(bets: Vec<Bet>) -> Vec<Statistic> {
    let mut stats: HashMap<String, Vec<Bet>> = HashMap::new();
    let mut total_amount = 0.0;

    for bet in bets {
        let stat = stats
            .entry(bet.candidate_id.value.clone())
            .or_insert_with(|| Vec::new());
        total_amount += bet.amount as f64;
        stat.push(bet);
    }

    let mut result = Vec::new();
    for (candidate_id, bets) in stats {
        let amount = bets.iter().map(|b| b.amount).sum();
        let rate = total_amount / amount as f64;
        result.push(Statistic {
            candidate_id: Id::new(candidate_id),
            amount,
            rate,
            bets,
        });
    }

    // amount が大きい順にソート
    result.sort_by(|a, b| b.amount.cmp(&a.amount));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_statistics() {
        fn random_string() -> String {
            use random_string::generate;
            generate(10, "abcdefghijklmnopqrstuvwxyz")
        }
        fn get_match_id() -> Id<Match> {
            Id::new("match_id".to_string())
        }
        fn get_bet_candidate_1(amount: i32) -> Bet {
            Bet::new(
                Id::new(random_string()),
                Id::new(random_string()),
                get_match_id(),
                Id::new("candidate_1".to_string()),
                amount,
            )
        }
        fn get_bet_candidate_2(amount: i32) -> Bet {
            Bet::new(
                Id::new(random_string()),
                Id::new(random_string()),
                get_match_id(),
                Id::new("candidate_2".to_string()),
                amount,
            )
        }

        let bets = vec![
            get_bet_candidate_1(100),
            get_bet_candidate_1(200),
            get_bet_candidate_1(300),
            get_bet_candidate_2(400),
            get_bet_candidate_2(500),
        ];

        let stats = calculate_statistics(bets);

        assert_eq!(stats.len(), 2);
        assert_eq!(stats[0].candidate_id.value, "candidate_2");
        assert_eq!(stats[0].amount, 900);
        assert!((stats[0].rate - 1.666).abs() < 0.001); // approximately equal
        assert_eq!(stats[1].candidate_id.value, "candidate_1");
        assert_eq!(stats[1].amount, 600);
        assert!((stats[1].rate - 2.5).abs() < 0.001); // approximately equal
    }
}

#[derive(new, Debug)]
pub struct NewBetForLatestMatch {
    pub id: Id<Bet>,
    pub traq_id: String,
    pub channel_id: String,
    pub candidate_name: String,
    pub amount: i32,
}
