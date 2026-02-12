use rand::Rng;
use rust_i18n::t;

use crate::game::balance::{
    BOSS_ATK_PER_LEVEL, BOSS_BASE_ATK, BOSS_BASE_DEF, BOSS_BASE_EXP, BOSS_BASE_GOLD, BOSS_BASE_HP,
    BOSS_DEF_PER_LEVEL, BOSS_EXP_PER_LEVEL, BOSS_GOLD_PER_LEVEL, BOSS_HP_PER_LEVEL, BOSS_NAME_KEY,
    NORMAL_ENEMIES, NORMAL_ENEMY_ATK_PER_LEVEL, NORMAL_ENEMY_DEF_PER_LEVEL,
    NORMAL_ENEMY_EXP_PER_LEVEL, NORMAL_ENEMY_GOLD_PER_LEVEL, NORMAL_ENEMY_HP_PER_LEVEL,
};
use crate::game::model::Enemy;

pub fn generate_enemy(rng: &mut impl Rng, player_level: i32, boss: bool) -> Enemy {
    if boss {
        let hp = BOSS_BASE_HP + player_level * BOSS_HP_PER_LEVEL;
        return Enemy {
            name: t!(BOSS_NAME_KEY).to_string(),
            hp,
            max_hp: hp,
            atk: BOSS_BASE_ATK + player_level * BOSS_ATK_PER_LEVEL,
            def: BOSS_BASE_DEF + player_level * BOSS_DEF_PER_LEVEL,
            exp_reward: BOSS_BASE_EXP + player_level * BOSS_EXP_PER_LEVEL,
            gold_reward: BOSS_BASE_GOLD + player_level * BOSS_GOLD_PER_LEVEL,
            is_boss: true,
        };
    }

    let scale = player_level - 1;
    let idx = rng.random_range(0..NORMAL_ENEMIES.len());
    let template = &NORMAL_ENEMIES[idx];
    Enemy {
        name: t!(template.name_key).to_string(),
        hp: template.base_hp + scale * NORMAL_ENEMY_HP_PER_LEVEL,
        max_hp: template.base_hp + scale * NORMAL_ENEMY_HP_PER_LEVEL,
        atk: template.base_atk + scale * NORMAL_ENEMY_ATK_PER_LEVEL,
        def: template.base_def + scale * NORMAL_ENEMY_DEF_PER_LEVEL,
        exp_reward: template.base_exp + scale * NORMAL_ENEMY_EXP_PER_LEVEL,
        gold_reward: template.base_gold + scale * NORMAL_ENEMY_GOLD_PER_LEVEL,
        is_boss: false,
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use super::generate_enemy;

    #[test]
    fn boss_generation_uses_boss_identity() {
        rust_i18n::set_locale("en");
        let mut rng = StdRng::seed_from_u64(1);
        let enemy = generate_enemy(&mut rng, 3, true);
        assert!(enemy.is_boss);
        assert!(!enemy.name.is_empty());
        assert!(enemy.hp > 0);
    }

    #[test]
    fn normal_enemy_scales_with_level() {
        rust_i18n::set_locale("en");
        let mut rng1 = StdRng::seed_from_u64(9);
        let mut rng2 = StdRng::seed_from_u64(9);
        let e1 = generate_enemy(&mut rng1, 1, false);
        let e2 = generate_enemy(&mut rng2, 3, false);

        assert!(!e1.is_boss);
        assert!(!e2.is_boss);
        assert_eq!(e1.name, e2.name);
        assert!(e2.hp > e1.hp);
        assert!(e2.atk > e1.atk);
        assert!(e2.exp_reward > e1.exp_reward);
    }
}
