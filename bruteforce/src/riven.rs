use std::slice::Iter;

use itertools::Itertools;
use wf_mods::melee::*;
use wf_stats::*;

pub enum RivenAttribute {
    CriticalChance(f32),
    CriticalMultiplier(f32),
    Damage(f32),
    Status(Status),
    StatusChance(f32),
    AttackSpeed(f32),
}

impl RivenAttribute {
    pub fn bonus_list(disposition: f32, bonus: f32) -> [RivenAttribute; 9] {
        [
            RivenAttribute::CriticalChance(1.80 * disposition * bonus),
            RivenAttribute::CriticalMultiplier(0.9 * disposition * bonus),
            RivenAttribute::Damage(1.647 * disposition * bonus),
            RivenAttribute::Status(Status::cold(0.9 * disposition * bonus)),
            RivenAttribute::Status(Status::electricity(0.9 * disposition * bonus)),
            RivenAttribute::Status(Status::heat(0.9 * disposition * bonus)),
            RivenAttribute::Status(Status::toxin(0.9 * disposition * bonus)),
            RivenAttribute::StatusChance(0.9 * disposition * bonus),
            RivenAttribute::AttackSpeed(0.549 * disposition * bonus),
        ]
    }

    pub fn to_melee_riven(attributes: Iter<&RivenAttribute>) -> MeleeMod {
        let mut critical_chance = 0.0;
        let mut critical_multiplier = 0.0;
        let mut damage = 0.0;
        let mut status_chance = 0.0;
        let mut attack_speed = 0.0;
        let mut status_list = Vec::new();

        for attribute in attributes {
            match attribute {
                RivenAttribute::CriticalChance(value) => critical_chance = *value,
                RivenAttribute::CriticalMultiplier(value) => critical_multiplier = *value,
                RivenAttribute::Damage(value) => damage = *value,
                RivenAttribute::Status(status) => status_list.push(status.clone()),
                RivenAttribute::StatusChance(value) => status_chance = *value,
                RivenAttribute::AttackSpeed(value) => attack_speed = *value,
            }
        }

        MeleeMod::Riven(MeleeRiven::new(
            damage,
            critical_chance,
            critical_multiplier,
            status_chance,
            attack_speed,
            status_list,
        ))
    }
}

/// Generate all possible riven combinations for a melee weapon
///
/// # Arguments
///
/// * `disposition` - The riven disposition of the weapon
/// * `attribute_count` - The number of attributes in the riven
/// * `bonus` - The bonus relative to the number of positive attributes (and negative attributes)
pub fn generate_riven_combinations(
    disposition: f32,
    attribute_count: usize,
    bonus: f32,
) -> Vec<MeleeMod> {
    RivenAttribute::bonus_list(disposition, bonus)
        .iter()
        .combinations(attribute_count)
        .map(|c| RivenAttribute::to_melee_riven(c.iter()))
        .collect()
}
