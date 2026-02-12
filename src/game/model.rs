#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    Town,
    Lair,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    Exploration,
    Town,
    Settings,
    Battle,
    Victory,
    GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub exp_reward: i32,
    pub gold_reward: i32,
    pub is_boss: bool,
}

pub struct Battle {
    pub enemy: Enemy,
    pub defending: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy)]
pub struct Equipment {
    pub weapon: WeaponTier,
    pub armor: ArmorTier,
}

#[derive(Clone, Copy)]
pub struct Bag {
    pub potion: i32,
    pub ether: i32,
}

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

pub const MAP_W: usize = 36;
pub const MAP_H: usize = 18;
pub const LOG_CAPACITY: usize = 10;

#[cfg(test)]
mod tests {
    use super::Language;

    #[test]
    fn locale_tag_mapping_supports_new_languages() {
        assert_eq!(Language::from_locale_tag("zh-TW"), Language::ZhTw);
        assert_eq!(Language::from_locale_tag("zh-HK"), Language::ZhTw);
        assert_eq!(Language::from_locale_tag("ko-KR"), Language::Ko);
        assert_eq!(Language::from_locale_tag("ja-JP"), Language::Ja);
        assert_eq!(Language::from_locale_tag("en"), Language::En);
    }
}
