use crate::game::balance::{
    LEVEL_UP_ATK_INCREASE, LEVEL_UP_DEF_INCREASE, LEVEL_UP_HP_INCREASE, LEVEL_UP_MP_INCREASE,
    NEXT_EXP_BASE_INCREASE, NEXT_EXP_LEVEL_MULTIPLIER,
};
use crate::game::model::{Enemy, Player};
use rust_i18n::t;

pub fn apply_battle_rewards(player: &mut Player, enemy: &Enemy) -> Vec<String> {
    let mut logs = Vec::new();
    player.exp += enemy.exp_reward;
    player.gold += enemy.gold_reward;
    logs.push(
        t!(
            "log.progression.defeated_reward",
            enemy = enemy.name.as_str(),
            exp = enemy.exp_reward,
            gold = enemy.gold_reward
        )
        .to_string(),
    );

    while player.exp >= player.next_exp {
        player.exp -= player.next_exp;
        player.level += 1;
        player.next_exp += NEXT_EXP_BASE_INCREASE + player.level * NEXT_EXP_LEVEL_MULTIPLIER;
        player.max_hp += LEVEL_UP_HP_INCREASE;
        player.max_mp += LEVEL_UP_MP_INCREASE;
        player.base_atk += LEVEL_UP_ATK_INCREASE;
        player.base_def += LEVEL_UP_DEF_INCREASE;
        player.hp = player.max_hp;
        player.mp = player.max_mp;
        logs.push(t!("log.progression.level_up", level = player.level).to_string());
    }

    logs
}

#[cfg(test)]
mod tests {
    use super::apply_battle_rewards;
    use crate::game::model::{Enemy, Player};

    fn enemy(exp_reward: i32, gold_reward: i32) -> Enemy {
        Enemy {
            name: "Test".to_string(),
            hp: 1,
            max_hp: 1,
            atk: 1,
            def: 1,
            exp_reward,
            gold_reward,
            is_boss: false,
        }
    }

    #[test]
    fn reward_adds_gold_and_exp() {
        rust_i18n::set_locale("en");
        let mut player = Player::new();
        player.exp = 0;
        player.gold = 0;

        let logs = apply_battle_rewards(&mut player, &enemy(5, 9));
        assert!(!logs.is_empty());
        assert_eq!(player.gold, 9);
        assert_eq!(player.exp, 5);
    }

    #[test]
    fn enough_exp_triggers_level_up_and_restore() {
        rust_i18n::set_locale("en");
        let mut player = Player::new();
        player.exp = 19;
        player.next_exp = 20;
        player.hp = 1;
        player.mp = 1;

        let _ = apply_battle_rewards(&mut player, &enemy(5, 0));
        assert_eq!(player.level, 2);
        assert_eq!(player.hp, player.max_hp);
        assert_eq!(player.mp, player.max_mp);
    }
}
