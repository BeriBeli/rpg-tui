pub struct EnemyTemplate {
    pub name_key: &'static str,
    pub base_hp: i32,
    pub base_atk: i32,
    pub base_def: i32,
    pub base_exp: i32,
    pub base_gold: i32,
}

pub const RANDOM_ENCOUNTER_RATE_PERCENT: i32 = 16;
pub const WORLD_EVENT_RATE_PERCENT: i32 = 14;

pub const NORMAL_ENEMIES: [EnemyTemplate; 5] = [
    EnemyTemplate {
        name_key: "enemy.slime",
        base_hp: 18,
        base_atk: 6,
        base_def: 1,
        base_exp: 8,
        base_gold: 6,
    },
    EnemyTemplate {
        name_key: "enemy.goblin",
        base_hp: 24,
        base_atk: 8,
        base_def: 2,
        base_exp: 11,
        base_gold: 8,
    },
    EnemyTemplate {
        name_key: "enemy.wolf",
        base_hp: 20,
        base_atk: 10,
        base_def: 2,
        base_exp: 13,
        base_gold: 11,
    },
    EnemyTemplate {
        name_key: "enemy.skeleton",
        base_hp: 26,
        base_atk: 9,
        base_def: 3,
        base_exp: 16,
        base_gold: 14,
    },
    EnemyTemplate {
        name_key: "enemy.orc_brute",
        base_hp: 34,
        base_atk: 12,
        base_def: 4,
        base_exp: 20,
        base_gold: 18,
    },
];

pub const NORMAL_ENEMY_HP_PER_LEVEL: i32 = 5;
pub const NORMAL_ENEMY_ATK_PER_LEVEL: i32 = 2;
pub const NORMAL_ENEMY_DEF_PER_LEVEL: i32 = 1;
pub const NORMAL_ENEMY_EXP_PER_LEVEL: i32 = 3;
pub const NORMAL_ENEMY_GOLD_PER_LEVEL: i32 = 3;

pub const BOSS_NAME_KEY: &str = "enemy.ancient_dragon";
pub const BOSS_BASE_HP: i32 = 84;
pub const BOSS_BASE_ATK: i32 = 16;
pub const BOSS_BASE_DEF: i32 = 7;
pub const BOSS_BASE_EXP: i32 = 55;
pub const BOSS_BASE_GOLD: i32 = 90;
pub const BOSS_HP_PER_LEVEL: i32 = 9;
pub const BOSS_ATK_PER_LEVEL: i32 = 2;
pub const BOSS_DEF_PER_LEVEL: i32 = 1;
pub const BOSS_EXP_PER_LEVEL: i32 = 8;
pub const BOSS_GOLD_PER_LEVEL: i32 = 10;

pub const NEXT_EXP_BASE_INCREASE: i32 = 12;
pub const NEXT_EXP_LEVEL_MULTIPLIER: i32 = 6;
pub const LEVEL_UP_HP_INCREASE: i32 = 6;
pub const LEVEL_UP_MP_INCREASE: i32 = 2;
pub const LEVEL_UP_ATK_INCREASE: i32 = 2;
pub const LEVEL_UP_DEF_INCREASE: i32 = 1;

pub const EVENT_WEIGHT_GOLD_CACHE: i32 = 30;
pub const EVENT_WEIGHT_POTION_STASH: i32 = 22;
pub const EVENT_WEIGHT_ETHER_STASH: i32 = 18;
pub const EVENT_WEIGHT_CAMPFIRE: i32 = 16;
pub const EVENT_WEIGHT_SPIKE_TRAP: i32 = 14;

pub const EVENT_GOLD_MIN: i32 = 7;
pub const EVENT_GOLD_MAX: i32 = 16;
pub const EVENT_GOLD_PER_LEVEL: i32 = 2;

pub const EVENT_POTION_MIN: i32 = 1;
pub const EVENT_POTION_MAX: i32 = 2;
pub const EVENT_ETHER_MIN: i32 = 1;
pub const EVENT_ETHER_MAX: i32 = 2;

pub const EVENT_CAMPFIRE_HEAL_BASE: i32 = 8;
pub const EVENT_CAMPFIRE_HEAL_PER_LEVEL: i32 = 2;

pub const EVENT_TRAP_DAMAGE_MIN: i32 = 4;
pub const EVENT_TRAP_DAMAGE_MAX: i32 = 10;
