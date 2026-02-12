use std::collections::VecDeque;

use crossterm::event::KeyCode;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rust_i18n::t;

use crate::game::battle::{self, BattleOutcome};
use crate::game::config::{self, DifficultyProfile};
use crate::game::encounter;
use crate::game::event;
use crate::game::model::{
    Battle, Difficulty, Enemy, GameMode, LOG_CAPACITY, Language, MAP_H, MAP_W, Player, Position,
    QuestState, Tile, WorldObjects,
};
use crate::game::progression;
use crate::game::save::{self, SaveData};
use crate::game::town::{self, TownAction, TownOutcome};
use crate::game::world::generate_world;

const RNG_SALT: u64 = 0x9E37_79B9_7F4A_7C15;

pub struct Game {
    pub mode: GameMode,
    pub map: Vec<Vec<Tile>>,
    pub world: WorldObjects,
    pub player: Player,
    pub battle: Option<Battle>,
    pub log: VecDeque<String>,
    pub should_quit: bool,
    pub current_language: Language,
    pub settings_cursor: usize,
    pub difficulty: Difficulty,
    pub map_seed: u64,
    pub recent_event: Option<String>,
    pub quest: QuestState,
    rng: StdRng,
    difficulty_profile: DifficultyProfile,
    settings_return_mode: GameMode,
    battle_origin: Option<Position>,
}

impl Game {
    pub fn new() -> Self {
        let initial_language = std::env::var("RPG_LANG")
            .ok()
            .as_deref()
            .map(Language::from_locale_tag)
            .unwrap_or(Language::En);
        let (difficulty, profile) = config::active_difficulty();
        let map_seed = rand::rng().random::<u64>();
        Self::new_with_setup(initial_language, difficulty, profile, map_seed)
    }

    pub fn new_with_seed(map_seed: u64) -> Self {
        let (difficulty, profile) = config::active_difficulty();
        Self::new_with_setup(Language::En, difficulty, profile, map_seed)
    }

    fn new_with_setup(
        language: Language,
        difficulty: Difficulty,
        difficulty_profile: DifficultyProfile,
        map_seed: u64,
    ) -> Self {
        rust_i18n::set_locale(language.locale_code());
        let (map, world) = generate_world(map_seed);
        let mut game = Self {
            mode: GameMode::Exploration,
            map,
            world,
            player: Player::new(),
            battle: None,
            log: VecDeque::new(),
            should_quit: false,
            current_language: language,
            settings_cursor: language.index(),
            difficulty,
            map_seed,
            recent_event: None,
            quest: QuestState::new(),
            rng: StdRng::seed_from_u64(map_seed ^ RNG_SALT),
            difficulty_profile,
            settings_return_mode: GameMode::Exploration,
            battle_origin: None,
        };
        game.push_log(t!("log.game.welcome"));
        game.push_log(t!("log.game.town_hint"));
        game.push_log(t!("log.game.difficulty", diff = t!(difficulty.label_key())));
        game
    }

    pub fn push_log(&mut self, msg: impl Into<String>) {
        if self.log.len() >= LOG_CAPACITY {
            self.log.pop_front();
        }
        self.log.push_back(msg.into());
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        if matches!(code, KeyCode::Char('q')) {
            self.should_quit = true;
            return;
        }
        if matches!(code, KeyCode::Esc) {
            if self.mode == GameMode::Settings {
                self.close_settings();
                return;
            }
            self.should_quit = true;
            return;
        }
        if matches!(code, KeyCode::Char('k')) {
            self.save_game();
            return;
        }
        if matches!(code, KeyCode::Char('l')) {
            self.load_game();
            return;
        }

        match self.mode {
            GameMode::Exploration => self.handle_exploration_key(code),
            GameMode::Town => self.handle_town_key(code),
            GameMode::Settings => self.handle_settings_key(code),
            GameMode::Battle => self.handle_battle_key(code),
            GameMode::Victory | GameMode::GameOver => {
                if matches!(code, KeyCode::Char('r')) {
                    self.restart();
                }
            }
        }
    }

    fn restart(&mut self) {
        let map_seed = rand::rng().random::<u64>();
        let profile = config::profile_for(self.difficulty);
        *self = Self::new_with_setup(self.current_language, self.difficulty, profile, map_seed);
    }

    fn save_game(&mut self) {
        let save = self.to_save_data();
        match save::save_to_default_file(&save) {
            Ok(path) => {
                let message = t!("log.game.saved_to", path = path.as_str()).to_string();
                self.recent_event = Some(message.clone());
                self.push_log(message);
            }
            Err(error) => {
                self.push_log(t!("log.game.save_failed", error = error.as_str()));
            }
        }
    }

    fn load_game(&mut self) {
        match save::load_from_default_file() {
            Ok((save_data, path)) => {
                let mut loaded = Self::from_save_data(save_data);
                let message = t!("log.game.loaded_from", path = path.as_str()).to_string();
                loaded.recent_event = Some(message.clone());
                loaded.push_log(message);
                *self = loaded;
            }
            Err(error) => {
                self.push_log(t!("log.game.load_failed", error = error.as_str()));
            }
        }
    }

    fn to_save_data(&self) -> SaveData {
        SaveData {
            version: 1,
            mode: self.mode,
            map_seed: self.map_seed,
            map: self.map.clone(),
            player: self.player.clone(),
            battle: self.battle.clone(),
            current_language: self.current_language,
            difficulty: self.difficulty,
            world: self.world.clone(),
            quest: self.quest.clone(),
            log: self.log.iter().cloned().collect(),
            recent_event: self.recent_event.clone(),
            battle_origin: self.battle_origin,
            settings_cursor: self.settings_cursor,
        }
    }

    fn from_save_data(save_data: SaveData) -> Self {
        let profile = config::profile_for(save_data.difficulty);
        rust_i18n::set_locale(save_data.current_language.locale_code());

        let mut game = Self {
            mode: save_data.mode,
            map: save_data.map,
            world: save_data.world,
            player: save_data.player,
            battle: save_data.battle,
            log: VecDeque::new(),
            should_quit: false,
            current_language: save_data.current_language,
            settings_cursor: save_data.settings_cursor,
            difficulty: save_data.difficulty,
            map_seed: save_data.map_seed,
            recent_event: save_data.recent_event,
            quest: save_data.quest,
            rng: StdRng::seed_from_u64(save_data.map_seed ^ RNG_SALT),
            difficulty_profile: profile,
            settings_return_mode: GameMode::Exploration,
            battle_origin: save_data.battle_origin,
        };

        for message in save_data.log.into_iter().rev().take(LOG_CAPACITY).rev() {
            game.push_log(message);
        }
        game
    }

    fn announce_event(&mut self, message: String) {
        self.recent_event = Some(message.clone());
        self.push_log(message);
    }

    fn handle_exploration_key(&mut self, code: KeyCode) {
        if matches!(code, KeyCode::Char('o')) {
            self.open_settings(GameMode::Exploration);
            return;
        }
        if matches!(code, KeyCode::Char('t')) && self.current_tile() == Tile::Town {
            self.mode = GameMode::Town;
            self.push_log(t!("log.town.menu_opened"));
            return;
        }

        let (dx, dy) = match code {
            KeyCode::Up | KeyCode::Char('w') => (0, -1),
            KeyCode::Down | KeyCode::Char('s') => (0, 1),
            KeyCode::Left | KeyCode::Char('a') => (-1, 0),
            KeyCode::Right | KeyCode::Char('d') => (1, 0),
            _ => return,
        };
        self.try_move_player(dx, dy);
    }

    fn handle_town_key(&mut self, code: KeyCode) {
        if matches!(code, KeyCode::Char('o')) {
            self.open_settings(GameMode::Town);
            return;
        }

        let action = match code {
            KeyCode::Char('1') => Some(TownAction::BuyPotion),
            KeyCode::Char('2') => Some(TownAction::BuyEther),
            KeyCode::Char('3') => Some(TownAction::UpgradeWeapon),
            KeyCode::Char('4') => Some(TownAction::UpgradeArmor),
            KeyCode::Char('5') => Some(TownAction::Healer),
            KeyCode::Char('6') => Some(TownAction::Inn),
            KeyCode::Char('7') => Some(TownAction::QuestBoard),
            KeyCode::Char('8') => Some(TownAction::Leave),
            _ => None,
        };

        let Some(action) = action else {
            return;
        };
        match town::apply_action(&mut self.player, &mut self.quest, action) {
            TownOutcome::Stay(message) => {
                self.recent_event = Some(message.clone());
                self.push_log(message);
            }
            TownOutcome::Leave(message) => {
                self.mode = GameMode::Exploration;
                self.push_log(message);
            }
        }
    }

    fn handle_settings_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('w') => {
                if self.settings_cursor == 0 {
                    self.settings_cursor = Language::ALL.len() - 1;
                } else {
                    self.settings_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.settings_cursor = (self.settings_cursor + 1) % Language::ALL.len();
            }
            KeyCode::Char('1') => self.select_language(0),
            KeyCode::Char('2') => self.select_language(1),
            KeyCode::Char('3') => self.select_language(2),
            KeyCode::Char('4') => self.select_language(3),
            KeyCode::Char('5') => self.select_language(4),
            KeyCode::Enter => self.select_language(self.settings_cursor),
            KeyCode::Char('b') => self.close_settings(),
            _ => {}
        }
    }

    fn handle_battle_key(&mut self, code: KeyCode) {
        let Some(action) = battle::action_from_key(code) else {
            return;
        };

        let Some(mut battle) = self.battle.take() else {
            self.mode = GameMode::Exploration;
            return;
        };

        let result = battle::resolve_turn(
            action,
            &mut battle,
            &mut self.player,
            &mut self.rng,
            &self.difficulty_profile,
        );
        for message in result.logs {
            self.push_log(message);
        }

        match result.outcome {
            BattleOutcome::Continue => {
                self.mode = GameMode::Battle;
                self.battle = Some(battle);
            }
            BattleOutcome::Escaped => {
                self.mode = GameMode::Exploration;
                self.battle = None;
                self.battle_origin = None;
            }
            BattleOutcome::EnemyDefeated(enemy) => self.win_battle(enemy),
            BattleOutcome::PlayerDefeated => {
                self.mode = GameMode::GameOver;
                self.battle = None;
                self.battle_origin = None;
                self.push_log(t!("log.game.player_fallen_restart"));
            }
        }
    }

    fn try_move_player(&mut self, dx: i32, dy: i32) {
        let nx = self.player.x as i32 + dx;
        let ny = self.player.y as i32 + dy;
        if nx < 0 || ny < 0 || nx as usize >= MAP_W || ny as usize >= MAP_H {
            return;
        }

        let nx = nx as usize;
        let ny = ny as usize;
        if self.map[ny][nx] == Tile::Wall {
            return;
        }

        self.player.x = nx;
        self.player.y = ny;
        match self.map[ny][nx] {
            Tile::Town => {
                self.player.hp = self.player.max_hp;
                self.player.mp = self.player.max_mp;
                self.mode = GameMode::Town;
                self.push_log(t!("log.town.arrived_restore"));
            }
            Tile::Lair => self.start_boss_battle(),
            Tile::Floor => self.handle_floor_tile(nx, ny),
            Tile::Wall => {}
        }
    }

    fn handle_floor_tile(&mut self, x: usize, y: usize) {
        if self.interact_chest(x, y) {
            return;
        }
        if self.interact_npc(x, y) {
            return;
        }
        if self.world.tile_is_cleared(x, y) {
            return;
        }

        let encounter_rate = self
            .difficulty_profile
            .random_encounter_rate_percent
            .clamp(0, 100);
        if self.rng.random_range(0..100) < encounter_rate {
            self.start_random_battle(Position { x, y });
            return;
        }

        if let Some(event_result) = event::maybe_trigger_event(
            &mut self.rng,
            &mut self.player,
            self.difficulty_profile.world_event_rate_percent,
        ) {
            self.announce_event(event_result.message);
            if event_result.player_dead {
                self.mode = GameMode::GameOver;
                self.battle = None;
                self.push_log(t!("log.game.player_fallen_restart"));
                return;
            }
            self.world.mark_tile_cleared(x, y);
        }
    }

    fn interact_chest(&mut self, x: usize, y: usize) -> bool {
        let rewards = {
            let Some(chest) = self.world.chest_at_mut(x, y) else {
                return false;
            };
            if chest.opened {
                return false;
            }
            chest.opened = true;
            Some((chest.gold, chest.potion, chest.ether))
        };

        let Some((gold, potion, ether)) = rewards else {
            return false;
        };
        self.player.gold += gold;
        self.player.bag.potion += potion;
        self.player.bag.ether += ether;
        self.world.mark_tile_cleared(x, y);
        self.announce_event(
            t!(
                "log.world.chest_opened",
                gold = gold,
                potion = potion,
                ether = ether
            )
            .to_string(),
        );
        true
    }

    fn interact_npc(&mut self, x: usize, y: usize) -> bool {
        let interaction = {
            let Some(npc) = self.world.npc_at_mut(x, y) else {
                return false;
            };
            if npc.interacted {
                return false;
            }
            npc.interacted = true;
            Some((npc.kind, npc.reward_gold))
        };

        let Some((kind, reward_gold)) = interaction else {
            return false;
        };
        self.player.gold += reward_gold;
        self.world.mark_tile_cleared(x, y);
        self.push_log(t!(kind.line_key()).to_string());
        self.announce_event(t!("log.world.npc_reward", gold = reward_gold).to_string());
        true
    }

    fn start_random_battle(&mut self, origin: Position) {
        let enemy = encounter::generate_enemy(
            &mut self.rng,
            self.player.level,
            false,
            &self.difficulty_profile,
        );
        self.push_log(t!("log.battle.wild_appears", enemy = enemy.name.as_str()));
        self.battle = Some(Battle {
            enemy,
            defending: false,
        });
        self.mode = GameMode::Battle;
        self.battle_origin = Some(origin);
    }

    fn start_boss_battle(&mut self) {
        let enemy = encounter::generate_enemy(
            &mut self.rng,
            self.player.level,
            true,
            &self.difficulty_profile,
        );
        self.push_log(t!(
            "log.battle.boss_blocks_path",
            enemy = enemy.name.as_str()
        ));
        self.battle = Some(Battle {
            enemy,
            defending: false,
        });
        self.mode = GameMode::Battle;
        self.battle_origin = None;
    }

    fn win_battle(&mut self, enemy: Enemy) {
        self.battle = None;
        self.battle_origin
            .take()
            .map(|origin| self.world.mark_tile_cleared(origin.x, origin.y));

        let reward_logs = progression::apply_battle_rewards(&mut self.player, &enemy);
        if let Some(first) = reward_logs.first() {
            self.recent_event = Some(first.clone());
        }
        for log in reward_logs {
            self.push_log(log);
        }

        if !enemy.is_boss {
            if self.quest.register_kill() {
                self.announce_event(
                    t!(
                        "log.quest.completed",
                        progress = self.quest.progress_text(),
                        reward = self.quest.reward_gold
                    )
                    .to_string(),
                );
            } else if self.quest.accepted && !self.quest.completed {
                self.push_log(
                    t!(
                        "log.quest.progress_short",
                        progress = self.quest.progress_text()
                    )
                    .to_string(),
                );
            }
            self.mode = GameMode::Exploration;
            return;
        }

        self.mode = GameMode::Victory;
        self.announce_event(t!("log.game.dragon_defeated_restart_or_quit").to_string());
    }

    fn current_tile(&self) -> Tile {
        self.map[self.player.y][self.player.x]
    }

    fn open_settings(&mut self, from_mode: GameMode) {
        self.settings_return_mode = from_mode;
        self.settings_cursor = self.current_language.index();
        self.mode = GameMode::Settings;
        self.push_log(t!("log.settings.opened"));
    }

    fn close_settings(&mut self) {
        let to = if self.settings_return_mode == GameMode::Settings {
            GameMode::Exploration
        } else {
            self.settings_return_mode
        };
        self.mode = to;
    }

    fn select_language(&mut self, idx: usize) {
        if idx >= Language::ALL.len() {
            return;
        }
        self.settings_cursor = idx;
        let lang = Language::ALL[idx];
        self.current_language = lang;
        rust_i18n::set_locale(lang.locale_code());
        self.push_log(t!(
            "log.settings.language_changed",
            lang = t!(lang.label_key())
        ));
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use super::Game;
    use crate::game::model::{GameMode, Position};

    #[test]
    fn town_purchase_through_handle_key_updates_player_state() {
        rust_i18n::set_locale("en");
        let mut game = Game::new_with_seed(7);
        game.mode = GameMode::Town;
        game.player.gold = 20;
        game.player.bag.potion = 0;

        game.handle_key(KeyCode::Char('1'));

        assert_eq!(game.player.gold, 10);
        assert_eq!(game.player.bag.potion, 1);
        assert_eq!(game.mode, GameMode::Town);
    }

    #[test]
    fn town_leave_through_handle_key_returns_to_exploration() {
        rust_i18n::set_locale("en");
        let mut game = Game::new_with_seed(11);
        game.mode = GameMode::Town;

        game.handle_key(KeyCode::Char('8'));

        assert_eq!(game.mode, GameMode::Exploration);
    }

    #[test]
    fn chest_interaction_is_one_time() {
        rust_i18n::set_locale("en");
        let mut game = Game::new_with_seed(42);
        let chest_pos = game.world.chests[0].position;
        game.player.x = chest_pos.x - 1;
        game.player.y = chest_pos.y;

        let before_gold = game.player.gold;
        game.try_move_player(1, 0);
        let after_first = game.player.gold;
        assert!(after_first > before_gold);

        game.try_move_player(-1, 0);
        game.try_move_player(1, 0);
        assert_eq!(game.player.gold, after_first);
    }

    #[test]
    fn full_flow_town_battle_level_up_boss_is_seed_deterministic() {
        rust_i18n::set_locale("en");
        let mut game = Game::new_with_seed(2026);

        game.handle_key(KeyCode::Right);
        game.handle_key(KeyCode::Down);
        assert_eq!(game.mode, GameMode::Town);

        game.player.gold = 100;
        game.handle_key(KeyCode::Char('1'));
        game.handle_key(KeyCode::Char('8'));
        assert_eq!(game.mode, GameMode::Exploration);

        game.player.exp = game.player.next_exp - 1;
        game.player.base_atk = 999;
        game.start_random_battle(Position { x: 4, y: 2 });
        assert_eq!(game.mode, GameMode::Battle);
        game.handle_key(KeyCode::Char('1'));
        assert!(game.player.level >= 2);
        assert_eq!(game.mode, GameMode::Exploration);

        game.player.base_atk = 999;
        game.start_boss_battle();
        game.handle_key(KeyCode::Char('1'));
        assert_eq!(game.mode, GameMode::Victory);
    }
}
