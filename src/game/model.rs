use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Floor,
    Wall,
    Town,
    Lair,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    Exploration,
    Town,
    Settings,
    Battle,
    Victory,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    En,
    ZhCn,
    ZhTw,
    Ja,
    Ko,
}

impl Language {
    pub const ALL: [Language; 5] = [
        Language::En,
        Language::ZhCn,
        Language::ZhTw,
        Language::Ja,
        Language::Ko,
    ];

    pub fn locale_code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::ZhCn => "zh-CN",
            Self::ZhTw => "zh-TW",
            Self::Ja => "ja",
            Self::Ko => "ko",
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            Self::En => "ui.settings.lang_en",
            Self::ZhCn => "ui.settings.lang_zh_cn",
            Self::ZhTw => "ui.settings.lang_zh_tw",
            Self::Ja => "ui.settings.lang_ja",
            Self::Ko => "ui.settings.lang_ko",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::En => 0,
            Self::ZhCn => 1,
            Self::ZhTw => 2,
            Self::Ja => 3,
            Self::Ko => 4,
        }
    }

    pub fn from_locale_tag(tag: &str) -> Self {
        let normalized = tag.to_ascii_lowercase();
        if normalized.starts_with("zh-tw")
            || normalized.starts_with("zh_hk")
            || normalized.starts_with("zh-hk")
        {
            return Self::ZhTw;
        }
        if normalized.starts_with("zh") {
            return Self::ZhCn;
        }
        if normalized.starts_with("ja") {
            return Self::Ja;
        }
        if normalized.starts_with("ko") {
            return Self::Ko;
        }
        Self::En
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub const ALL: [Difficulty; 3] = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];

    pub fn from_tag(tag: &str) -> Self {
        match tag.to_ascii_lowercase().as_str() {
            "easy" => Self::Easy,
            "hard" => Self::Hard,
            _ => Self::Normal,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Self::Easy => 0,
            Self::Normal => 1,
            Self::Hard => 2,
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            Self::Easy => "ui.settings.diff_easy",
            Self::Normal => "ui.settings.diff_normal",
            Self::Hard => "ui.settings.diff_hard",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyStyle {
    Skirmisher,
    Brute,
    Caster,
    Predator,
    Undead,
    Boss,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub exp_reward: i32,
    pub gold_reward: i32,
    pub is_boss: bool,
    pub style: EnemyStyle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Battle {
    pub enemy: Enemy,
    pub defending: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponTier {
    WoodenSword,
    BronzeSword,
    KnightSword,
}

impl WeaponTier {
    pub fn i18n_key(self) -> &'static str {
        match self {
            Self::WoodenSword => "item.weapon.wooden_sword",
            Self::BronzeSword => "item.weapon.bronze_sword",
            Self::KnightSword => "item.weapon.knight_sword",
        }
    }

    pub fn bonus(self) -> i32 {
        match self {
            Self::WoodenSword => 0,
            Self::BronzeSword => 3,
            Self::KnightSword => 7,
        }
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Self::WoodenSword => Some(Self::BronzeSword),
            Self::BronzeSword => Some(Self::KnightSword),
            Self::KnightSword => None,
        }
    }

    pub fn upgrade_cost(self) -> Option<i32> {
        match self.next() {
            Some(Self::BronzeSword) => Some(30),
            Some(Self::KnightSword) => Some(85),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorTier {
    ClothArmor,
    ChainArmor,
    SteelArmor,
}

impl ArmorTier {
    pub fn i18n_key(self) -> &'static str {
        match self {
            Self::ClothArmor => "item.armor.cloth_armor",
            Self::ChainArmor => "item.armor.chain_armor",
            Self::SteelArmor => "item.armor.steel_armor",
        }
    }

    pub fn bonus(self) -> i32 {
        match self {
            Self::ClothArmor => 0,
            Self::ChainArmor => 2,
            Self::SteelArmor => 6,
        }
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Self::ClothArmor => Some(Self::ChainArmor),
            Self::ChainArmor => Some(Self::SteelArmor),
            Self::SteelArmor => None,
        }
    }

    pub fn upgrade_cost(self) -> Option<i32> {
        match self.next() {
            Some(Self::ChainArmor) => Some(26),
            Some(Self::SteelArmor) => Some(80),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: WeaponTier,
    pub armor: ArmorTier,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Bag {
    pub potion: i32,
    pub ether: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub base_atk: i32,
    pub base_def: i32,
    pub level: i32,
    pub exp: i32,
    pub next_exp: i32,
    pub gold: i32,
    pub equipment: Equipment,
    pub bag: Bag,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 1,
            y: 1,
            hp: 40,
            max_hp: 40,
            mp: 12,
            max_mp: 12,
            base_atk: 10,
            base_def: 4,
            level: 1,
            exp: 0,
            next_exp: 20,
            gold: 15,
            equipment: Equipment {
                weapon: WeaponTier::WoodenSword,
                armor: ArmorTier::ClothArmor,
            },
            bag: Bag {
                potion: 1,
                ether: 1,
            },
        }
    }

    pub fn total_atk(&self) -> i32 {
        self.base_atk + self.equipment.weapon.bonus()
    }

    pub fn total_def(&self) -> i32 {
        self.base_def + self.equipment.armor.bonus()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcKind {
    Traveler,
    Scout,
    Sage,
}

impl NpcKind {
    pub fn line_key(self) -> &'static str {
        match self {
            Self::Traveler => "log.world.npc_traveler",
            Self::Scout => "log.world.npc_scout",
            Self::Sage => "log.world.npc_sage",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chest {
    pub position: Position,
    pub opened: bool,
    pub gold: i32,
    pub potion: i32,
    pub ether: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NpcPoint {
    pub position: Position,
    pub kind: NpcKind,
    pub interacted: bool,
    pub reward_gold: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldObjects {
    pub chests: Vec<Chest>,
    pub npcs: Vec<NpcPoint>,
    pub cleared_tiles: HashSet<Position>,
}

impl WorldObjects {
    pub fn new(chests: Vec<Chest>, npcs: Vec<NpcPoint>) -> Self {
        Self {
            chests,
            npcs,
            cleared_tiles: HashSet::new(),
        }
    }

    pub fn chest_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Chest> {
        self.chests
            .iter_mut()
            .find(|chest| chest.position.x == x && chest.position.y == y)
    }

    pub fn npc_at_mut(&mut self, x: usize, y: usize) -> Option<&mut NpcPoint> {
        self.npcs
            .iter_mut()
            .find(|npc| npc.position.x == x && npc.position.y == y)
    }

    pub fn tile_is_cleared(&self, x: usize, y: usize) -> bool {
        self.cleared_tiles.contains(&Position { x, y })
    }

    pub fn mark_tile_cleared(&mut self, x: usize, y: usize) {
        self.cleared_tiles.insert(Position { x, y });
    }

    pub fn has_unopened_chest(&self, x: usize, y: usize) -> bool {
        self.chests
            .iter()
            .any(|chest| !chest.opened && chest.position.x == x && chest.position.y == y)
    }

    pub fn has_active_npc(&self, x: usize, y: usize) -> bool {
        self.npcs
            .iter()
            .any(|npc| !npc.interacted && npc.position.x == x && npc.position.y == y)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestState {
    pub accepted: bool,
    pub completed: bool,
    pub rewarded: bool,
    pub kills: i32,
    pub target_kills: i32,
    pub reward_gold: i32,
}

impl QuestState {
    pub fn new() -> Self {
        Self {
            accepted: false,
            completed: false,
            rewarded: false,
            kills: 0,
            target_kills: 3,
            reward_gold: 45,
        }
    }

    pub fn register_kill(&mut self) -> bool {
        if !self.accepted || self.completed {
            return false;
        }
        self.kills += 1;
        if self.kills >= self.target_kills {
            self.completed = true;
            return true;
        }
        false
    }

    pub fn progress_text(&self) -> String {
        format!("{}/{}", self.kills, self.target_kills)
    }
}

impl Default for QuestState {
    fn default() -> Self {
        Self::new()
    }
}

pub const MAP_W: usize = 36;
pub const MAP_H: usize = 18;
pub const LOG_CAPACITY: usize = 10;

#[cfg(test)]
mod tests {
    use super::{Difficulty, Language, QuestState};

    #[test]
    fn locale_tag_mapping_supports_new_languages() {
        assert_eq!(Language::from_locale_tag("zh-TW"), Language::ZhTw);
        assert_eq!(Language::from_locale_tag("zh-HK"), Language::ZhTw);
        assert_eq!(Language::from_locale_tag("ko-KR"), Language::Ko);
        assert_eq!(Language::from_locale_tag("ja-JP"), Language::Ja);
        assert_eq!(Language::from_locale_tag("en"), Language::En);
    }

    #[test]
    fn difficulty_tag_mapping_works() {
        assert_eq!(Difficulty::from_tag("easy"), Difficulty::Easy);
        assert_eq!(Difficulty::from_tag("HARD"), Difficulty::Hard);
        assert_eq!(Difficulty::from_tag("unknown"), Difficulty::Normal);
    }

    #[test]
    fn quest_progress_marks_completion_at_target() {
        let mut quest = QuestState::new();
        quest.accepted = true;
        assert!(!quest.register_kill());
        assert!(!quest.register_kill());
        assert!(quest.register_kill());
        assert!(quest.completed);
    }
}
