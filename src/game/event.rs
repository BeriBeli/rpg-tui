use rand::Rng;
use rust_i18n::t;

use crate::game::balance::{
    EVENT_CAMPFIRE_HEAL_BASE, EVENT_CAMPFIRE_HEAL_PER_LEVEL, EVENT_ETHER_MAX, EVENT_ETHER_MIN,
    EVENT_GOLD_MAX, EVENT_GOLD_MIN, EVENT_GOLD_PER_LEVEL, EVENT_POTION_MAX, EVENT_POTION_MIN,
    EVENT_TRAP_DAMAGE_MAX, EVENT_TRAP_DAMAGE_MIN, EVENT_WEIGHT_CAMPFIRE, EVENT_WEIGHT_ETHER_STASH,
    EVENT_WEIGHT_GOLD_CACHE, EVENT_WEIGHT_POTION_STASH, EVENT_WEIGHT_SPIKE_TRAP,
};
use crate::game::model::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldEventKind {
    GoldCache,
    PotionStash,
    EtherStash,
    Campfire,
    SpikeTrap,
}

pub struct WorldEventResult {
    pub message: String,
    pub player_dead: bool,
}

pub fn maybe_trigger_event(
    rng: &mut impl Rng,
    player: &mut Player,
    event_rate_percent: i32,
) -> Option<WorldEventResult> {
    if rng.random_range(0..100) >= event_rate_percent.clamp(0, 100) {
        return None;
    }
    let kind = roll_event_kind(rng);
    Some(apply_event(kind, rng, player))
}

fn roll_event_kind(rng: &mut impl Rng) -> WorldEventKind {
    let total = EVENT_WEIGHT_GOLD_CACHE
        + EVENT_WEIGHT_POTION_STASH
        + EVENT_WEIGHT_ETHER_STASH
        + EVENT_WEIGHT_CAMPFIRE
        + EVENT_WEIGHT_SPIKE_TRAP;
    let roll = rng.random_range(0..total);
    event_kind_from_roll(roll)
}

fn event_kind_from_roll(roll: i32) -> WorldEventKind {
    if roll < EVENT_WEIGHT_GOLD_CACHE {
        return WorldEventKind::GoldCache;
    }
    if roll < EVENT_WEIGHT_GOLD_CACHE + EVENT_WEIGHT_POTION_STASH {
        return WorldEventKind::PotionStash;
    }
    if roll < EVENT_WEIGHT_GOLD_CACHE + EVENT_WEIGHT_POTION_STASH + EVENT_WEIGHT_ETHER_STASH {
        return WorldEventKind::EtherStash;
    }
    if roll
        < EVENT_WEIGHT_GOLD_CACHE
            + EVENT_WEIGHT_POTION_STASH
            + EVENT_WEIGHT_ETHER_STASH
            + EVENT_WEIGHT_CAMPFIRE
    {
        return WorldEventKind::Campfire;
    }
    WorldEventKind::SpikeTrap
}

fn apply_event(kind: WorldEventKind, rng: &mut impl Rng, player: &mut Player) -> WorldEventResult {
    match kind {
        WorldEventKind::GoldCache => {
            let gold = rng.random_range(EVENT_GOLD_MIN..=EVENT_GOLD_MAX)
                + player.level * EVENT_GOLD_PER_LEVEL;
            player.gold += gold;
            WorldEventResult {
                message: t!("log.event.gold_cache", gold = gold).to_string(),
                player_dead: false,
            }
        }
        WorldEventKind::PotionStash => {
            let amount = rng.random_range(EVENT_POTION_MIN..=EVENT_POTION_MAX);
            player.bag.potion += amount;
            WorldEventResult {
                message: t!("log.event.potion_stash", count = amount).to_string(),
                player_dead: false,
            }
        }
        WorldEventKind::EtherStash => {
            let amount = rng.random_range(EVENT_ETHER_MIN..=EVENT_ETHER_MAX);
            player.bag.ether += amount;
            WorldEventResult {
                message: t!("log.event.ether_stash", count = amount).to_string(),
                player_dead: false,
            }
        }
        WorldEventKind::Campfire => {
            let heal = EVENT_CAMPFIRE_HEAL_BASE + player.level * EVENT_CAMPFIRE_HEAL_PER_LEVEL;
            let before = player.hp;
            player.hp = (player.hp + heal).min(player.max_hp);
            WorldEventResult {
                message: t!(
                    "log.event.campfire_heal",
                    before = before,
                    after = player.hp
                )
                .to_string(),
                player_dead: false,
            }
        }
        WorldEventKind::SpikeTrap => {
            let damage = rng.random_range(EVENT_TRAP_DAMAGE_MIN..=EVENT_TRAP_DAMAGE_MAX);
            player.hp -= damage;
            if player.hp <= 0 {
                player.hp = 0;
                return WorldEventResult {
                    message: t!("log.event.spike_trap_deadly", dmg = damage).to_string(),
                    player_dead: true,
                };
            }
            WorldEventResult {
                message: t!("log.event.spike_trap", dmg = damage).to_string(),
                player_dead: false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use super::{WorldEventKind, apply_event, event_kind_from_roll, maybe_trigger_event};
    use crate::game::balance::{
        EVENT_WEIGHT_CAMPFIRE, EVENT_WEIGHT_ETHER_STASH, EVENT_WEIGHT_GOLD_CACHE,
        EVENT_WEIGHT_POTION_STASH,
    };
    use crate::game::model::Player;

    #[test]
    fn event_roll_boundaries_map_to_expected_kinds() {
        assert_eq!(event_kind_from_roll(0), WorldEventKind::GoldCache);
        assert_eq!(
            event_kind_from_roll(EVENT_WEIGHT_GOLD_CACHE),
            WorldEventKind::PotionStash
        );
        assert_eq!(
            event_kind_from_roll(EVENT_WEIGHT_GOLD_CACHE + EVENT_WEIGHT_POTION_STASH),
            WorldEventKind::EtherStash
        );
        assert_eq!(
            event_kind_from_roll(
                EVENT_WEIGHT_GOLD_CACHE + EVENT_WEIGHT_POTION_STASH + EVENT_WEIGHT_ETHER_STASH
            ),
            WorldEventKind::Campfire
        );
        assert_eq!(
            event_kind_from_roll(
                EVENT_WEIGHT_GOLD_CACHE
                    + EVENT_WEIGHT_POTION_STASH
                    + EVENT_WEIGHT_ETHER_STASH
                    + EVENT_WEIGHT_CAMPFIRE
            ),
            WorldEventKind::SpikeTrap
        );
    }

    #[test]
    fn campfire_never_overheals() {
        rust_i18n::set_locale("en");
        let mut rng = StdRng::seed_from_u64(10);
        let mut player = Player::new();
        player.hp = player.max_hp - 1;
        let result = apply_event(WorldEventKind::Campfire, &mut rng, &mut player);
        assert!(!result.player_dead);
        assert_eq!(player.hp, player.max_hp);
    }

    #[test]
    fn spike_trap_can_kill_player() {
        rust_i18n::set_locale("en");
        let mut rng = StdRng::seed_from_u64(1);
        let mut player = Player::new();
        player.hp = 1;
        let result = apply_event(WorldEventKind::SpikeTrap, &mut rng, &mut player);
        assert!(result.player_dead);
        assert_eq!(player.hp, 0);
    }

    #[test]
    fn zero_event_rate_never_triggers() {
        rust_i18n::set_locale("en");
        let mut rng = StdRng::seed_from_u64(5);
        let mut player = Player::new();
        assert!(maybe_trigger_event(&mut rng, &mut player, 0).is_none());
    }
}
