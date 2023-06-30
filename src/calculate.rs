use crate::app::{WarOdds, WarWeights, MAX_SOLDIERS};

pub fn get_combinations(my_count: usize, kills: usize) -> f64 {
    let mut on_balance = 1.;
    for i in 0..kills {
        on_balance *= (my_count - i) as f64 / (kills - i) as f64;
    }
    on_balance.round()
}

fn odds_of_kills(my_count: usize, rate: f64, kills: usize) -> f64 {
    if kills > my_count {
        return 0.;
    }
    let combinations = get_combinations(my_count, kills);
    let kill = rate.powi(kills as i32);
    let no_kill = (1. - rate).powi((my_count - kills) as i32);
    combinations * kill * no_kill
}

fn normalize(weights: &mut WarWeights) {
    let sum: f64 = weights.0.iter().sum();
    for weight in weights.0.iter_mut() {
        *weight /= sum;
    }
}

fn step_battle(weights: &WarWeights, odds: &WarOdds) -> WarWeights {
    let mut attacker_weights = WarWeights::default();

    for attackers in 0..=MAX_SOLDIERS {
        for defenders in 0..=MAX_SOLDIERS {
            let slot = WarWeights::slot_for(attackers, defenders);
            let weight = weights.0[slot];
            if attackers == 0 || defenders == 0 {
                attacker_weights.0[slot] += weight;
                continue;
            }
            for attacker_kills in 0..=(22.min(attackers)) {
                let engagements = if odds.limited_engagements != 0 { attackers.min(odds.limited_engagements) } else { attackers };
                let chance = odds_of_kills(engagements, odds.get_attacker_rate(), attacker_kills);
                let new_defenders = if defenders > attacker_kills { defenders - attacker_kills } else { 0 };
                attacker_weights.0[WarWeights::slot_for(attackers, new_defenders)] += weight * chance;
            }
        }
    }

    normalize(&mut attacker_weights);

    let mut new_weights = WarWeights::default();

    for attackers in 0..=MAX_SOLDIERS {
        for defenders in 0..=MAX_SOLDIERS {
            let slot = WarWeights::slot_for(attackers, defenders);
            let weight = attacker_weights.0[slot];
            if attackers == 0 || defenders == 0 {
                new_weights.0[slot] += weight;
                continue;
            }
            for defender_kills in 0..=(22.min(defenders)) {
                let engagements = if odds.limited_engagements != 0 { defenders.min(odds.limited_engagements) } else { defenders };
                let chance: f64 = odds_of_kills(engagements, odds.get_defender_rate(), defender_kills);
                let new_attackers = if attackers > defender_kills { attackers - defender_kills } else { 0 };
                new_weights.0[WarWeights::slot_for(new_attackers, defenders)] += weight * chance;
            }
        }
    }

    normalize(&mut new_weights);

    new_weights
}

pub fn calculate_weights(starting_attackers: f64, starting_defenders: f64, odds: &WarOdds) -> WarWeights {
    let mut weights = WarWeights::default();
    weights.0[WarWeights::slot_for(starting_attackers as usize, starting_defenders as usize)] = 1.;
    for _ in 0..odds.round_count {
        weights = step_battle(&weights, odds);
    }
    weights
}

mod tests {
    use super::*;

    #[test]
    fn test_get_combinations() {
        // assert_eq!(get_combinations(100, 0), 1.);
        assert_eq!(get_combinations(100, 1), 100.);
        assert_eq!(get_combinations(100, 2), 4950.);
        assert_eq!(get_combinations(100, 3), 161700.);
        assert_eq!(get_combinations(100, 4), 3921225.);
        assert_eq!(get_combinations(100, 5), 75287520.);
        assert_eq!(get_combinations(100, 6), 1192052400.);
        assert_eq!(get_combinations(100, 7), 16007560800.);
        assert_eq!(get_combinations(100, 8), 186087894300.);
        assert_eq!(get_combinations(100, 9), 1902231808400.);
        assert_eq!(get_combinations(100, 10), 17310309456440.);
        assert_eq!(get_combinations(100, 11), 141629804643600.);
        assert_eq!(get_combinations(100, 12), 1050421051106700.0);
        assert_eq!(get_combinations(100, 13), 7110542499799199.0);
        assert_eq!(get_combinations(100, 14), 4.41869426773236e16);
        assert_eq!(get_combinations(100, 15), 2.5333847134998864e17);
        assert_eq!(get_combinations(100, 16), 1.3458606290468147e18);
        assert_eq!(get_combinations(100, 17), 6.650134872937204e18);
        assert_eq!(get_combinations(100, 18), 3.0664510802988196e19);
        assert_eq!(get_combinations(100, 19), 1.323415729392123e20);
        assert_eq!(get_combinations(100, 20), 5.359833704038098e20);
    }

    #[test]
    fn test_odds_of_kills() {
        assert_eq!(odds_of_kills(100, 0.1, 1), 0.0002951266543065283);
        assert_eq!(odds_of_kills(100, 0.1, 2), 0.0016231965986859057);
        assert_eq!(odds_of_kills(100, 0.1, 3), 0.005891602469304398);
        assert_eq!(odds_of_kills(100, 0.1, 4), 0.015874595542292404);
        assert_eq!(odds_of_kills(100, 0.1, 5), 0.03386580382355713);
        assert_eq!(odds_of_kills(100, 0.1, 6), 0.05957872894885052);
        assert_eq!(odds_of_kills(100, 0.1, 7), 0.08889524636812617);
        assert_eq!(odds_of_kills(100, 0.1, 8), 0.11482302655882966);
        assert_eq!(odds_of_kills(100, 0.1, 9), 0.13041627707916453);
        assert_eq!(odds_of_kills(100, 0.1, 10), 0.13186534682448858);
        assert_eq!(odds_of_kills(100, 0.1, 11), 0.11987758802226234);
        assert_eq!(odds_of_kills(100, 0.1, 12), 0.09878801235167915);
        assert_eq!(odds_of_kills(100, 0.1, 13), 0.0743020947602373);
        assert_eq!(odds_of_kills(100, 0.1, 14), 0.05130382733444958);
        assert_eq!(odds_of_kills(100, 0.1, 15), 0.03268243815379751);
        assert_eq!(odds_of_kills(100, 0.1, 16), 0.019291716965783256);
        assert_eq!(odds_of_kills(100, 0.1, 17), 0.010591530883175122);
        assert_eq!(odds_of_kills(100, 0.1, 18), 0.005426525082120584);
        assert_eq!(odds_of_kills(100, 0.1, 19), 0.0026021933142332644);
        assert_eq!(odds_of_kills(100, 0.1, 20), 0.001170986991404969);
    }

    #[test]
    fn test_step_battle() {
        let mut weights = WarWeights::default();
        weights.0[WarWeights::slot_for(100, 100)] = 1.;
        let odds = WarOdds::default();
        let new_weights = step_battle(&weights, &odds);
        assert_eq!(new_weights.0[WarWeights::slot_for(90, 86)], 0.0097634899509056);
    }
}