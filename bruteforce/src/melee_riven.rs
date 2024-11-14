use itertools::Itertools;
use wf_mods::melee::*;
use wf_stats::*;

const MELEE_RIVEN_BASE_BONUS_LIST: [MeleeRivenAttribute; 9] = [
    MeleeRivenAttribute::CriticalChance(1.80),
    MeleeRivenAttribute::CriticalMultiplier(0.9),
    MeleeRivenAttribute::Damage(1.647),
    MeleeRivenAttribute::Status(Status::cold(0.9)),
    MeleeRivenAttribute::Status(Status::electricity(0.9)),
    MeleeRivenAttribute::Status(Status::heat(0.9)),
    MeleeRivenAttribute::Status(Status::toxin(0.9)),
    MeleeRivenAttribute::StatusChance(0.9),
    MeleeRivenAttribute::AttackSpeed(0.549),
];

#[derive(Clone, Debug)]
pub enum MeleeRivenAttribute {
    CriticalChance(f32),
    CriticalMultiplier(f32),
    Damage(f32),
    Status(Status),
    StatusChance(f32),
    AttackSpeed(f32),
}

impl std::ops::Mul<f32> for MeleeRivenAttribute {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        use MeleeRivenAttribute::*;

        match self {
            CriticalChance(value) => CriticalChance(value * rhs),
            CriticalMultiplier(value) => CriticalMultiplier(value * rhs),
            Damage(value) => Damage(value * rhs),
            Status(status) => Status(status * rhs),
            StatusChance(value) => StatusChance(value * rhs),
            AttackSpeed(value) => AttackSpeed(value * rhs),
        }
    }
}

impl<'a> FromIterator<&'a MeleeRivenAttribute> for MeleeMod {
    fn from_iter<T: IntoIterator<Item = &'a MeleeRivenAttribute>>(iter: T) -> Self {
        use MeleeRivenAttribute::*;

        let mut critical_chance = 0.0;
        let mut critical_multiplier = 0.0;
        let mut damage = 0.0;
        let mut status_chance = 0.0;
        let mut attack_speed = 0.0;
        let mut status_list = Vec::new();

        for attribute in iter {
            match attribute {
                CriticalChance(value) => critical_chance = *value,
                CriticalMultiplier(value) => critical_multiplier = *value,
                Damage(value) => damage = *value,
                Status(status) => status_list.push(*status),
                StatusChance(value) => status_chance = *value,
                AttackSpeed(value) => attack_speed = *value,
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
    MELEE_RIVEN_BASE_BONUS_LIST
        .map(|attribute| attribute * disposition * bonus)
        .iter()
        .combinations(attribute_count)
        .map(|c| c.into_iter().collect())
        .collect()
}
