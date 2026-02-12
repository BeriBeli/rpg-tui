use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::game::model::Difficulty;

const DEFAULT_DIFFICULTY_CONFIG_PATH: &str = "config/difficulty.toml";

#[derive(Clone, Debug, Deserialize)]
pub struct DifficultyProfile {
    pub random_encounter_rate_percent: i32,
    pub world_event_rate_percent: i32,
    pub enemy_hp_scale: f32,
    pub enemy_atk_scale: f32,
    pub enemy_def_scale: f32,
    pub enemy_reward_scale: f32,
    pub enemy_skill_rate_percent: i32,
    pub run_chance_bonus_percent: i32,
}

impl DifficultyProfile {
    pub fn scale_stat(&self, value: i32, scale: f32) -> i32 {
        ((value as f32 * scale).round() as i32).max(1)
    }

    pub fn clamp_rate(value: i32) -> i32 {
        value.clamp(0, 100)
    }
}

#[derive(Clone, Debug, Deserialize)]
struct DifficultyProfiles {
    easy: DifficultyProfile,
    normal: DifficultyProfile,
    hard: DifficultyProfile,
}

impl DifficultyProfiles {
    fn defaults() -> Self {
        Self {
            easy: DifficultyProfile {
                random_encounter_rate_percent: 12,
                world_event_rate_percent: 18,
                enemy_hp_scale: 0.86,
                enemy_atk_scale: 0.85,
                enemy_def_scale: 0.9,
                enemy_reward_scale: 0.95,
                enemy_skill_rate_percent: 16,
                run_chance_bonus_percent: 18,
            },
            normal: DifficultyProfile {
                random_encounter_rate_percent: 16,
                world_event_rate_percent: 14,
                enemy_hp_scale: 1.0,
                enemy_atk_scale: 1.0,
                enemy_def_scale: 1.0,
                enemy_reward_scale: 1.0,
                enemy_skill_rate_percent: 26,
                run_chance_bonus_percent: 0,
            },
            hard: DifficultyProfile {
                random_encounter_rate_percent: 21,
                world_event_rate_percent: 11,
                enemy_hp_scale: 1.22,
                enemy_atk_scale: 1.18,
                enemy_def_scale: 1.15,
                enemy_reward_scale: 1.12,
                enemy_skill_rate_percent: 40,
                run_chance_bonus_percent: -10,
            },
        }
    }

    fn profile(&self, difficulty: Difficulty) -> DifficultyProfile {
        match difficulty {
            Difficulty::Easy => self.easy.clone(),
            Difficulty::Normal => self.normal.clone(),
            Difficulty::Hard => self.hard.clone(),
        }
    }
}

fn load_profiles(path: &Path) -> DifficultyProfiles {
    let Ok(content) = fs::read_to_string(path) else {
        return DifficultyProfiles::defaults();
    };
    toml::from_str(&content).unwrap_or_else(|_| DifficultyProfiles::defaults())
}

pub fn profile_for(difficulty: Difficulty) -> DifficultyProfile {
    let profiles = load_profiles(Path::new(DEFAULT_DIFFICULTY_CONFIG_PATH));
    profiles.profile(difficulty)
}
