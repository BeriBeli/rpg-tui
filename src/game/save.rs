use std::fs;

use serde::{Deserialize, Serialize};

use crate::game::model::{
    Battle, Difficulty, GameMode, Language, Player, Position, QuestState, Tile, WorldObjects,
};

const SAVE_FILE_VERSION: u32 = 1;

#[derive(Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub mode: GameMode,
    pub map_seed: u64,
    pub map: Vec<Vec<Tile>>,
    pub player: Player,
    pub battle: Option<Battle>,
    pub current_language: Language,
    pub difficulty: Difficulty,
    pub world: WorldObjects,
    pub quest: QuestState,
    pub log: Vec<String>,
    pub recent_event: Option<String>,
    pub battle_origin: Option<Position>,
    pub settings_cursor: usize,
}

impl SaveData {
    fn path() -> String {
        std::env::var("RPG_SAVE_PATH").unwrap_or_else(|_| "savegame.json".to_string())
    }
}

pub fn save_to_default_file(save: &SaveData) -> Result<String, String> {
    let path = SaveData::path();
    save_to_path(save, path.as_str())?;
    Ok(path)
}

pub fn load_from_default_file() -> Result<(SaveData, String), String> {
    let path = SaveData::path();
    let save = load_from_path(path.as_str())?;
    Ok((save, path))
}

pub fn save_to_path(save: &SaveData, path: &str) -> Result<(), String> {
    let content = serde_json::to_string_pretty(save).map_err(|err| err.to_string())?;
    fs::write(path, content).map_err(|err| err.to_string())?;
    Ok(())
}

pub fn load_from_path(path: &str) -> Result<SaveData, String> {
    let content = fs::read_to_string(path).map_err(|err| err.to_string())?;
    let save: SaveData = serde_json::from_str(&content).map_err(|err| err.to_string())?;
    if save.version != SAVE_FILE_VERSION {
        return Err(format!(
            "unsupported save version: {} (expected {})",
            save.version, SAVE_FILE_VERSION
        ));
    }
    Ok(save)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{SaveData, load_from_path, save_to_path};
    use crate::game::model::{
        Difficulty, GameMode, Language, Player, Position, QuestState, WorldObjects,
    };

    fn temp_save_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "rpg_tui_save_test_{}_{}.json",
            std::process::id(),
            rand::random::<u64>()
        );
        path.push(unique);
        path
    }

    #[test]
    fn save_round_trip_restores_exact_state() {
        let path = temp_save_path();
        let mut player = Player::new();
        player.x = 7;
        player.y = 9;
        player.gold = 123;

        let save = SaveData {
            version: 1,
            mode: GameMode::Battle,
            map_seed: 88,
            map: vec![vec![crate::game::model::Tile::Floor; 4]; 3],
            player,
            battle: None,
            current_language: Language::Ja,
            difficulty: Difficulty::Hard,
            world: WorldObjects::new(Vec::new(), Vec::new()),
            quest: QuestState::new(),
            log: vec!["a".to_string(), "b".to_string()],
            recent_event: Some("recent".to_string()),
            battle_origin: Some(Position { x: 7, y: 9 }),
            settings_cursor: 3,
        };

        save_to_path(&save, path.to_string_lossy().as_ref()).expect("save should succeed");
        let loaded = load_from_path(path.to_string_lossy().as_ref()).expect("load should succeed");

        assert_eq!(loaded.mode, GameMode::Battle);
        assert_eq!(loaded.map_seed, 88);
        assert_eq!(loaded.player.x, 7);
        assert_eq!(loaded.player.y, 9);
        assert_eq!(loaded.player.gold, 123);
        assert_eq!(loaded.current_language, Language::Ja);
        assert_eq!(loaded.difficulty, Difficulty::Hard);
        assert_eq!(loaded.settings_cursor, 3);
        assert_eq!(loaded.recent_event.as_deref(), Some("recent"));
        assert_eq!(loaded.log.len(), 2);

        let _ = std::fs::remove_file(path);
    }
}
