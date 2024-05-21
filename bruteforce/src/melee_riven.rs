use std::slice::Iter;

use itertools::Itertools;
use wf_mods::melee::*;
use wf_stats::*;

pub enum MeleeRivenAttribute {
    CriticalChance(f32),
    CriticalMultiplier(f32),
    Damage(f32),
    Status(Status),
    StatusChance(f32),
    AttackSpeed(f32),
}

impl MeleeRivenAttribute {
    pub fn bonus_list(disposition: f32, bonus: f32) -> [MeleeRivenAttribute; 9] {
        [
            MeleeRivenAttribute::CriticalChance(1.80 * disposition * bonus),
            MeleeRivenAttribute::CriticalMultiplier(0.9 * disposition * bonus),
            MeleeRivenAttribute::Damage(1.647 * disposition * bonus),
            MeleeRivenAttribute::Status(Status::cold(0.9 * disposition * bonus)),
            MeleeRivenAttribute::Status(Status::electricity(0.9 * disposition * bonus)),
            MeleeRivenAttribute::Status(Status::heat(0.9 * disposition * bonus)),
            MeleeRivenAttribute::Status(Status::toxin(0.9 * disposition * bonus)),
            MeleeRivenAttribute::StatusChance(0.9 * disposition * bonus),
            MeleeRivenAttribute::AttackSpeed(0.549 * disposition * bonus),
        ]
    }

    pub fn to_melee_riven(attributes: Iter<&MeleeRivenAttribute>) -> MeleeMod {
        let mut critical_chance = 0.0;
        let mut critical_multiplier = 0.0;
        let mut damage = 0.0;
        let mut status_chance = 0.0;
        let mut attack_speed = 0.0;
        let mut status_list = Vec::new();

        for attribute in attributes {
            match attribute {
                MeleeRivenAttribute::CriticalChance(value) => critical_chance = *value,
                MeleeRivenAttribute::CriticalMultiplier(value) => critical_multiplier = *value,
                MeleeRivenAttribute::Damage(value) => damage = *value,
                MeleeRivenAttribute::Status(status) => status_list.push(status.clone()),
                MeleeRivenAttribute::StatusChance(value) => status_chance = *value,
                MeleeRivenAttribute::AttackSpeed(value) => attack_speed = *value,
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
pub fn generate_melee_riven_combinations(
    disposition: f32,
    attribute_count: usize,
    bonus: f32,
) -> Vec<MeleeMod> {
    MeleeRivenAttribute::bonus_list(disposition, bonus)
        .iter()
        .combinations(attribute_count)
        .map(|c| MeleeRivenAttribute::to_melee_riven(c.iter()))
        .collect()
}
