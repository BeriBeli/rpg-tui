use crate::game::model::Player;
use rust_i18n::t;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TownAction {
    BuyPotion,
    BuyEther,
    UpgradeWeapon,
    UpgradeArmor,
    Leave,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TownOutcome {
    Stay(String),
    Leave(String),
}

pub fn apply_action(player: &mut Player, action: TownAction) -> TownOutcome {
    match action {
        TownAction::BuyPotion => {
            if player.gold < 10 {
                return TownOutcome::Stay(t!("log.town.not_enough_gold_potion").to_string());
            }
            player.gold -= 10;
            player.bag.potion += 1;
            TownOutcome::Stay(t!("log.town.bought_potion", count = 1).to_string())
        }
        TownAction::BuyEther => {
            if player.gold < 12 {
                return TownOutcome::Stay(t!("log.town.not_enough_gold_ether").to_string());
            }
            player.gold -= 12;
            player.bag.ether += 1;
            TownOutcome::Stay(t!("log.town.bought_ether", count = 1).to_string())
        }
        TownAction::UpgradeWeapon => {
            let weapon = player.equipment.weapon;
            let Some(next_weapon) = weapon.next() else {
                return TownOutcome::Stay(t!("log.town.weapon_max").to_string());
            };
            let Some(cost) = weapon.upgrade_cost() else {
                return TownOutcome::Stay(t!("log.town.weapon_upgrade_unavailable").to_string());
            };
            if player.gold < cost {
                return TownOutcome::Stay(
                    t!("log.town.need_more_gold_weapon", cost = cost).to_string(),
                );
            }

            player.gold -= cost;
            player.equipment.weapon = next_weapon;
            TownOutcome::Stay(
                t!(
                    "log.town.weapon_upgraded",
                    weapon = t!(next_weapon.i18n_key())
                )
                .to_string(),
            )
        }
        TownAction::UpgradeArmor => {
            let armor = player.equipment.armor;
            let Some(next_armor) = armor.next() else {
                return TownOutcome::Stay(t!("log.town.armor_max").to_string());
            };
            let Some(cost) = armor.upgrade_cost() else {
                return TownOutcome::Stay(t!("log.town.armor_upgrade_unavailable").to_string());
            };
            if player.gold < cost {
                return TownOutcome::Stay(
                    t!("log.town.need_more_gold_armor", cost = cost).to_string(),
                );
            }

            player.gold -= cost;
            player.equipment.armor = next_armor;
            TownOutcome::Stay(
                t!("log.town.armor_upgraded", armor = t!(next_armor.i18n_key())).to_string(),
            )
        }
        TownAction::Leave => TownOutcome::Leave(t!("log.town.leaving").to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{TownAction, TownOutcome, apply_action};
    use crate::game::model::{ArmorTier, Player, WeaponTier};

    #[test]
    fn buy_potion_updates_inventory_and_gold() {
        rust_i18n::set_locale("en");
        let mut player = Player::new();
        player.gold = 30;
        player.bag.potion = 0;

        let out = apply_action(&mut player, TownAction::BuyPotion);
        assert!(matches!(out, TownOutcome::Stay(_)));
        assert_eq!(player.gold, 20);
        assert_eq!(player.bag.potion, 1);
    }

    #[test]
    fn upgrade_weapon_moves_to_next_tier_and_costs_gold() {
        rust_i18n::set_locale("en");
        let mut player = Player::new();
        player.gold = 100;
        player.equipment.weapon = WeaponTier::WoodenSword;

        let out = apply_action(&mut player, TownAction::UpgradeWeapon);
        assert!(matches!(out, TownOutcome::Stay(_)));
        assert_eq!(player.equipment.weapon, WeaponTier::BronzeSword);
        assert_eq!(player.gold, 70);
    }

    #[test]
    fn upgrade_armor_fails_when_not_enough_gold() {
        rust_i18n::set_locale("en");
        let mut player = Player::new();
        player.gold = 5;
        player.equipment.armor = ArmorTier::ClothArmor;

        let out = apply_action(&mut player, TownAction::UpgradeArmor);
        assert!(matches!(out, TownOutcome::Stay(_)));
        assert_eq!(player.equipment.armor, ArmorTier::ClothArmor);
        assert_eq!(player.gold, 5);
    }
}
