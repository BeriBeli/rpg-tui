use crossterm::event::KeyCode;
use rand::Rng;
use rust_i18n::t;

use crate::game::combat;
use crate::game::model::{Battle, Enemy, Player};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattleAction {
    Attack,
    FireSlash,
    Defend,
    Potion,
    Ether,
    Run,
}

pub enum BattleOutcome {
    Continue,
    Escaped,
    EnemyDefeated(Enemy),
    PlayerDefeated,
}

pub struct TurnResult {
    pub outcome: BattleOutcome,
    pub logs: Vec<String>,
}

pub fn action_from_key(code: KeyCode) -> Option<BattleAction> {
    match code {
        KeyCode::Char('1') => Some(BattleAction::Attack),
        KeyCode::Char('2') => Some(BattleAction::FireSlash),
        KeyCode::Char('3') => Some(BattleAction::Defend),
        KeyCode::Char('4') => Some(BattleAction::Potion),
        KeyCode::Char('5') => Some(BattleAction::Ether),
        KeyCode::Char('6') => Some(BattleAction::Run),
        _ => None,
    }
}

pub fn resolve_turn<R: Rng>(
    action: BattleAction,
    battle: &mut Battle,
    player: &mut Player,
    rng: &mut R,
) -> TurnResult {
    let mut logs = Vec::new();
    let mut player_acted = false;

    match action {
        BattleAction::Attack => {
            let dmg = combat::random_damage(rng, player.total_atk(), battle.enemy.def, 3);
            battle.enemy.hp -= dmg;
            logs.push(
                t!(
                    "log.battle.player_slash",
                    enemy = battle.enemy.name.as_str(),
                    dmg = dmg
                )
                .to_string(),
            );
            player_acted = true;
        }
        BattleAction::FireSlash => {
            if player.mp < 4 {
                logs.push(t!("log.battle.not_enough_mp_fire_slash").to_string());
            } else {
                player.mp -= 4;
                let dmg = combat::random_damage(rng, player.total_atk() + 6, battle.enemy.def, 5);
                battle.enemy.hp -= dmg;
                logs.push(
                    t!(
                        "log.battle.fire_slash",
                        enemy = battle.enemy.name.as_str(),
                        dmg = dmg
                    )
                    .to_string(),
                );
                player_acted = true;
            }
        }
        BattleAction::Defend => {
            battle.defending = true;
            logs.push(t!("log.battle.brace").to_string());
            player_acted = true;
        }
        BattleAction::Potion => {
            if use_potion(player, &mut logs) {
                player_acted = true;
            }
        }
        BattleAction::Ether => {
            if use_ether(player, &mut logs) {
                player_acted = true;
            }
        }
        BattleAction::Run => {
            let chance = if battle.enemy.is_boss { 12 } else { 45 };
            if rng.random_range(0..100) < chance {
                logs.push(t!("log.battle.escape_success").to_string());
                return TurnResult {
                    outcome: BattleOutcome::Escaped,
                    logs,
                };
            }
            logs.push(t!("log.battle.escape_failed").to_string());
            player_acted = true;
        }
    }

    if battle.enemy.hp <= 0 {
        return TurnResult {
            outcome: BattleOutcome::EnemyDefeated(battle.enemy.clone()),
            logs,
        };
    }

    if player_acted {
        let raw = combat::random_damage(rng, battle.enemy.atk, player.total_def(), 2);
        let dealt = if battle.defending {
            (raw / 2).max(1)
        } else {
            raw
        };
        player.hp -= dealt;
        logs.push(
            t!(
                "log.battle.enemy_hit",
                enemy = battle.enemy.name.as_str(),
                dmg = dealt
            )
            .to_string(),
        );
        battle.defending = false;

        if player.hp <= 0 {
            player.hp = 0;
            return TurnResult {
                outcome: BattleOutcome::PlayerDefeated,
                logs,
            };
        }
    }

    TurnResult {
        outcome: BattleOutcome::Continue,
        logs,
    }
}

fn use_potion(player: &mut Player, logs: &mut Vec<String>) -> bool {
    if player.bag.potion <= 0 {
        logs.push(t!("log.item.no_potion").to_string());
        return false;
    }
    if player.hp >= player.max_hp {
        logs.push(t!("log.item.hp_full").to_string());
        return false;
    }
    player.bag.potion -= 1;
    let before = player.hp;
    player.hp = (player.hp + 20).min(player.max_hp);
    logs.push(t!("log.item.potion_used", before = before, after = player.hp).to_string());
    true
}

fn use_ether(player: &mut Player, logs: &mut Vec<String>) -> bool {
    if player.bag.ether <= 0 {
        logs.push(t!("log.item.no_ether").to_string());
        return false;
    }
    if player.mp >= player.max_mp {
        logs.push(t!("log.item.mp_full").to_string());
        return false;
    }
    player.bag.ether -= 1;
    let before = player.mp;
    player.mp = (player.mp + 8).min(player.max_mp);
    logs.push(t!("log.item.ether_used", before = before, after = player.mp).to_string());
    true
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use super::{BattleAction, BattleOutcome, action_from_key, resolve_turn};
    use crate::game::model::{Battle, Enemy, Player};
    use crossterm::event::KeyCode;

    fn sample_enemy() -> Enemy {
        Enemy {
            name: "Test Enemy".to_string(),
            hp: 20,
            max_hp: 20,
            atk: 8,
            def: 2,
            exp_reward: 1,
            gold_reward: 1,
            is_boss: false,
        }
    }

    #[test]
    fn key_to_action_mapping_is_correct() {
        assert_eq!(
            action_from_key(KeyCode::Char('1')),
            Some(BattleAction::Attack)
        );
        assert_eq!(action_from_key(KeyCode::Char('6')), Some(BattleAction::Run));
        assert_eq!(action_from_key(KeyCode::Char('x')), None);
    }

    #[test]
    fn fire_slash_without_mp_does_not_trigger_enemy_attack() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut player = Player::new();
        player.hp = 30;
        player.mp = 0;
        let mut battle = Battle {
            enemy: sample_enemy(),
            defending: false,
        };

        let result = resolve_turn(BattleAction::FireSlash, &mut battle, &mut player, &mut rng);
        assert!(matches!(result.outcome, BattleOutcome::Continue));
        assert_eq!(player.hp, 30);
        assert_eq!(battle.enemy.hp, 20);
    }

    #[test]
    fn attack_can_finish_enemy_before_counterattack() {
        let mut rng = StdRng::seed_from_u64(9);
        let mut player = Player::new();
        player.base_atk = 99;
        let mut battle = Battle {
            enemy: Enemy {
                hp: 3,
                max_hp: 3,
                ..sample_enemy()
            },
            defending: false,
        };

        let result = resolve_turn(BattleAction::Attack, &mut battle, &mut player, &mut rng);
        assert!(matches!(result.outcome, BattleOutcome::EnemyDefeated(_)));
    }

    #[test]
    fn player_defeat_is_reported_and_hp_clamped_to_zero() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut player = Player::new();
        player.hp = 1;
        player.base_atk = 1;
        player.base_def = 0;
        let mut battle = Battle {
            enemy: Enemy {
                hp: 999,
                max_hp: 999,
                atk: 10,
                def: 50,
                ..sample_enemy()
            },
            defending: false,
        };

        let result = resolve_turn(BattleAction::Attack, &mut battle, &mut player, &mut rng);
        assert!(matches!(result.outcome, BattleOutcome::PlayerDefeated));
        assert_eq!(player.hp, 0);
    }
}
