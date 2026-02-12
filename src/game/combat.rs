use rand::Rng;

pub fn damage_with_roll(attack: i32, defense: i32, roll: i32) -> i32 {
    (attack - defense + roll).max(1)
}

pub fn random_damage(rng: &mut impl Rng, attack: i32, defense: i32, variance: i32) -> i32 {
    let roll = rng.random_range(-variance..=variance);
    damage_with_roll(attack, defense, roll)
}

#[cfg(test)]
mod tests {
    use super::{damage_with_roll, random_damage};

    #[test]
    fn damage_has_minimum_one() {
        let dmg = damage_with_roll(3, 10, -2);
        assert_eq!(dmg, 1);
    }

    #[test]
    fn damage_applies_attack_defense_and_roll() {
        let dmg = damage_with_roll(20, 8, -1);
        assert_eq!(dmg, 11);
    }

    #[test]
    fn random_damage_respects_variance_bounds() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let dmg = random_damage(&mut rng, 10, 6, 2);
            assert!((2..=6).contains(&dmg));
        }
    }
}
