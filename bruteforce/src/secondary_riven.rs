use itertools::Itertools;
use wf_mods::secondary::*;
use wf_stats::*;

const SECONDARY_RIVEN_BASE_BONUS_LIST: [SecondaryRivenAttribute; 13] = [
    SecondaryRivenAttribute::CriticalChance(1.4999),
    SecondaryRivenAttribute::CriticalMultiplier(0.9),
    SecondaryRivenAttribute::Damage(2.196),
    SecondaryRivenAttribute::Status(Status::cold(0.9)),
    SecondaryRivenAttribute::Status(Status::electricity(0.9)),
    SecondaryRivenAttribute::Status(Status::heat(0.9)),
    SecondaryRivenAttribute::Status(Status::toxin(0.9)),
    SecondaryRivenAttribute::StatusChance(0.9),
    SecondaryRivenAttribute::FireRate(0.747),
    SecondaryRivenAttribute::AmmoMaximum(0.9),
    SecondaryRivenAttribute::MagazineCapacity(0.5),
    SecondaryRivenAttribute::Multishot(1.197),
    SecondaryRivenAttribute::ReloadSpeed(0.5),
];

#[derive(Clone, Debug)]
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

impl<'a> FromIterator<&'a SecondaryRivenAttribute> for SecondaryRiven {
    fn from_iter<T: IntoIterator<Item = &'a SecondaryRivenAttribute>>(iter: T) -> Self {
        use SecondaryRivenAttribute::*;

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

        for attribute in iter {
            match attribute {
                CriticalChance(value) => critical_chance = *value,
                CriticalMultiplier(value) => critical_multiplier = *value,
                Damage(value) => damage = *value,
                Status(status) => status_list.push(*status),
                StatusChance(value) => status_chance = *value,
                FireRate(value) => fire_rate = *value,
                AmmoMaximum(value) => ammo_maximum = *value,
                MagazineCapacity(value) => magazine_capacity = *value,
                Multishot(value) => multishot = *value,
                ReloadSpeed(value) => reload_speed = *value,
            }
        }

        SecondaryRiven::new(
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
        )
    }
}

/// Generate all possible riven combinations for a secondary weapon
///
/// # Arguments
///
/// * `disposition` - The riven disposition of the weapon
/// * `attribute_count` - The number of attributes in the riven
/// * `has_negative` - Whether the riven has a negative attribute (used for bonus calculation)
pub fn generate_secondary_riven_combinations(
    disposition: f32,
    attribute_count: usize,
    has_negative: bool,
) -> impl Iterator<Item = SecondaryMod> {
    let bonus = match (attribute_count, has_negative) {
        (2, false) => 0.99,
        (2, true) => 1.2375,
        (3, false) => 0.75,
        (3, true) => 0.9375,
        _ => panic!("Invalid attribute count"),
    };

    SECONDARY_RIVEN_BASE_BONUS_LIST
        .iter()
        .combinations(attribute_count)
        .map(|c| c.into_iter().collect())
        .map(move |riven: SecondaryRiven| SecondaryMod::Riven(riven * disposition * bonus))
}
