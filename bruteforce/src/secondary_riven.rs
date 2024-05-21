use std::slice::Iter;

use itertools::Itertools;
use wf_mods::secondary::*;
use wf_stats::*;

pub enum SecondaryRivenAttribute {
    Damage(f32),
    CriticalChance(f32),
    CriticalMultiplier(f32),
    Status(Status),
    StatusChance(f32),
    FireRate(f32),
    AmmoMaximum(f32),
    MagazineCapacity(f32),
    Multishot(f32),
    ReloadSpeed(f32),
}

impl SecondaryRivenAttribute {
    pub fn bonus_list(disposition: f32, bonus: f32) -> [SecondaryRivenAttribute; 13] {
        [
            SecondaryRivenAttribute::CriticalChance(1.4999 * disposition * bonus),
            SecondaryRivenAttribute::CriticalMultiplier(0.9 * disposition * bonus),
            SecondaryRivenAttribute::Damage(2.196 * disposition * bonus),
            SecondaryRivenAttribute::Status(Status::cold(0.9 * disposition * bonus)),
            SecondaryRivenAttribute::Status(Status::electricity(0.9 * disposition * bonus)),
            SecondaryRivenAttribute::Status(Status::heat(0.9 * disposition * bonus)),
            SecondaryRivenAttribute::Status(Status::toxin(0.9 * disposition * bonus)),
            SecondaryRivenAttribute::StatusChance(0.9 * disposition * bonus),
            SecondaryRivenAttribute::FireRate(0.747 * disposition * bonus),
            SecondaryRivenAttribute::AmmoMaximum(0.9 * disposition * bonus),
            SecondaryRivenAttribute::MagazineCapacity(0.5 * disposition * bonus),
            SecondaryRivenAttribute::Multishot(1.197 * disposition * bonus),
            SecondaryRivenAttribute::ReloadSpeed(0.5 * disposition * bonus),
        ]
    }

    pub fn to_secondary_riven(attributes: Iter<&SecondaryRivenAttribute>) -> SecondaryMod {
        let mut critical_chance = 0.0;
        let mut critical_multiplier = 0.0;
        let mut damage = 0.0;
        let mut fire_rate = 0.0;
        let mut ammo_maximum = 0.0;
        let mut magazine_capacity = 0.0;
        let mut multishot = 0.0;
        let mut reload_speed = 0.0;
        let mut status_chance = 0.0;
        let mut status_list = Vec::new();

        for attribute in attributes {
            match attribute {
                SecondaryRivenAttribute::CriticalChance(value) => critical_chance = *value,
                SecondaryRivenAttribute::CriticalMultiplier(value) => critical_multiplier = *value,
                SecondaryRivenAttribute::Damage(value) => damage = *value,
                SecondaryRivenAttribute::Status(status) => status_list.push(status.clone()),
                SecondaryRivenAttribute::StatusChance(value) => status_chance = *value,
                SecondaryRivenAttribute::FireRate(value) => fire_rate = *value,
                SecondaryRivenAttribute::AmmoMaximum(value) => ammo_maximum = *value,
                SecondaryRivenAttribute::MagazineCapacity(value) => magazine_capacity = *value,
                SecondaryRivenAttribute::Multishot(value) => multishot = *value,
                SecondaryRivenAttribute::ReloadSpeed(value) => reload_speed = *value,
            }
        }

        SecondaryMod::Riven(SecondaryRiven::new(
            damage,
            critical_chance,
            critical_multiplier,
            fire_rate,
            ammo_maximum,
            magazine_capacity,
            multishot,
            reload_speed,
            status_chance,
            status_list,
        ))
    }
}

/// Generate all possible riven combinations for a secondary weapon
///
/// # Arguments
///
/// * `disposition` - The riven disposition of the weapon
/// * `attribute_count` - The number of attributes in the riven
/// * `bonus` - The bonus relative to the number of positive attributes (and negative attributes)
pub fn generate_secondary_riven_combinations(
    disposition: f32,
    attribute_count: usize,
    bonus: f32,
) -> Vec<SecondaryMod> {
    SecondaryRivenAttribute::bonus_list(disposition, bonus)
        .iter()
        .combinations(attribute_count)
        .map(|c| SecondaryRivenAttribute::to_secondary_riven(c.iter()))
        .collect()
}
