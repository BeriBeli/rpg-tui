use rand::Rng;
use rust_i18n::t;

use crate::game::balance::{
    BOSS_ATK_PER_LEVEL, BOSS_BASE_ATK, BOSS_BASE_DEF, BOSS_BASE_EXP, BOSS_BASE_GOLD, BOSS_BASE_HP,
    BOSS_DEF_PER_LEVEL, BOSS_EXP_PER_LEVEL, BOSS_GOLD_PER_LEVEL, BOSS_HP_PER_LEVEL, BOSS_NAME_KEY,
    NORMAL_ENEMIES, NORMAL_ENEMY_ATK_PER_LEVEL, NORMAL_ENEMY_DEF_PER_LEVEL,
    NORMAL_ENEMY_EXP_PER_LEVEL, NORMAL_ENEMY_GOLD_PER_LEVEL, NORMAL_ENEMY_HP_PER_LEVEL,
};
use crate::game::config::DifficultyProfile;
use crate::game::model::{Enemy, EnemyStyle};

pub fn generate_enemy(
    rng: &mut impl Rng,
    player_level: i32,
    boss: bool,
    difficulty: &DifficultyProfile,
) -> Enemy {
    if boss {
        let hp = difficulty.scale_stat(
            BOSS_BASE_HP + player_level * BOSS_HP_PER_LEVEL,
            difficulty.enemy_hp_scale,
        );
        return Enemy {
            name: t!(BOSS_NAME_KEY).to_string(),
            hp,
            max_hp: hp,
            atk: difficulty.scale_stat(
                BOSS_BASE_ATK + player_level * BOSS_ATK_PER_LEVEL,
                difficulty.enemy_atk_scale,
            ),
            def: difficulty.scale_stat(
                BOSS_BASE_DEF + player_level * BOSS_DEF_PER_LEVEL,
                difficulty.enemy_def_scale,
            ),
            exp_reward: difficulty.scale_stat(
                BOSS_BASE_EXP + player_level * BOSS_EXP_PER_LEVEL,
                difficulty.enemy_reward_scale,
            ),
            gold_reward: difficulty.scale_stat(
                BOSS_BASE_GOLD + player_level * BOSS_GOLD_PER_LEVEL,
                difficulty.enemy_reward_scale,
            ),
            is_boss: true,
            style: EnemyStyle::Boss,
        };
    }

    let scale = player_level - 1;
    let idx = rng.random_range(0..NORMAL_ENEMIES.len());
    let template = &NORMAL_ENEMIES[idx];
    let hp = difficulty.scale_stat(
        template.base_hp + scale * NORMAL_ENEMY_HP_PER_LEVEL,
        difficulty.enemy_hp_scale,
    );
    Enemy {
        name: t!(template.name_key).to_string(),
        hp,
        max_hp: hp,
        atk: difficulty.scale_stat(
            template.base_atk + scale * NORMAL_ENEMY_ATK_PER_LEVEL,
            difficulty.enemy_atk_scale,
        ),
        def: difficulty.scale_stat(
            template.base_def + scale * NORMAL_ENEMY_DEF_PER_LEVEL,
            difficulty.enemy_def_scale,
        ),
        exp_reward: difficulty.scale_stat(
            template.base_exp + scale * NORMAL_ENEMY_EXP_PER_LEVEL,
            difficulty.enemy_reward_scale,
        ),
        gold_reward: difficulty.scale_stat(
            template.base_gold + scale * NORMAL_ENEMY_GOLD_PER_LEVEL,
            difficulty.enemy_reward_scale,
        ),
        is_boss: false,
        style: template.style,
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use super::generate_enemy;
    use crate::game::config::profile_for;
    use crate::game::model::Difficulty;

    #[test]
    fn boss_generation_uses_boss_identity() {
        rust_i18n::set_locale("en");
        let mut rng = StdRng::seed_from_u64(1);
        let profile = profile_for(Difficulty::Normal);
        let enemy = generate_enemy(&mut rng, 3, true, &profile);
        assert!(enemy.is_boss);
        assert!(!enemy.name.is_empty());
        assert!(enemy.hp > 0);
    }

    #[test]
    fn normal_enemy_scales_with_level() {
        rust_i18n::set_locale("en");
        let mut rng1 = StdRng::seed_from_u64(9);
        let mut rng2 = StdRng::seed_from_u64(9);
        let profile = profile_for(Difficulty::Normal);
        let e1 = generate_enemy(&mut rng1, 1, false, &profile);
        let e2 = generate_enemy(&mut rng2, 3, false, &profile);

        assert!(!e1.is_boss);
        assert!(!e2.is_boss);
        assert_eq!(e1.name, e2.name);
        assert!(e2.hp > e1.hp);
        assert!(e2.atk > e1.atk);
        assert!(e2.exp_reward > e1.exp_reward);
    }
}
