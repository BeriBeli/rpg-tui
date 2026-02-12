use crossterm::event::KeyCode;

use rpg_tui::game::Game;
use rpg_tui::game::model::{Battle, Enemy, EnemyStyle, GameMode, MAP_H, MAP_W};

#[test]
fn seeded_flow_town_battle_level_up_boss_victory() {
    rust_i18n::set_locale("en");
    let mut game = Game::new_with_seed(2026);

    game.handle_key(KeyCode::Right);
    game.handle_key(KeyCode::Down);
    assert_eq!(game.mode, GameMode::Town);

    game.player.gold = 100;
    game.handle_key(KeyCode::Char('1'));
    game.handle_key(KeyCode::Char('8'));
    assert_eq!(game.mode, GameMode::Exploration);
    assert!(game.player.bag.potion >= 1);

    game.player.exp = game.player.next_exp - 1;
    game.player.base_atk = 999;
    game.mode = GameMode::Battle;
    game.battle = Some(Battle {
        enemy: Enemy {
            name: "Dummy".to_string(),
            hp: 4,
            max_hp: 4,
            atk: 1,
            def: 0,
            exp_reward: 2,
            gold_reward: 1,
            is_boss: false,
            style: EnemyStyle::Skirmisher,
        },
        defending: false,
    });
    game.handle_key(KeyCode::Char('1'));
    assert!(game.player.level >= 2);
    assert_eq!(game.mode, GameMode::Exploration);

    game.player.base_atk = 999;
    game.player.x = MAP_W - 3;
    game.player.y = MAP_H - 2;
    game.handle_key(KeyCode::Right);
    assert_eq!(game.mode, GameMode::Battle);
    game.handle_key(KeyCode::Char('1'));
    assert_eq!(game.mode, GameMode::Victory);
}
