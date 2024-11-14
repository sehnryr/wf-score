use std::sync::Arc;

use crate::status::Status;
use crate::weapon::Weapon;

pub trait WeaponModifiers {
    fn add_modifier(&mut self, modifier: Arc<dyn Modifier>)
    where
        Self: Weapon;
}

pub trait Modifier {
    /// Returns the name of this modifier's set if it is part of a set.
    ///
    /// Example:
    /// Gladiator Might: Gladiator set
    fn set(&self) -> Option<&str> {
        None
    }

    /// Returns the additive damage bonus of the modifier.
    ///
    /// Example:
    /// Primed Pressure Point: +165% damage bonus (1.65)
    // fn damage<W>(&self, context: W) -> f32
    // where
    //     W: Weapon + ?Sized;
    fn damage(&self, context: &dyn Weapon) -> f32;

    /// Returns the absolute anti-faction multiplier of the modifier.
    ///
    /// Example:
    /// Primed Smite Grineer: x1.55 anti-faction multiplier (0.55)
    fn anti_faction(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive critical chance bonus of the modifier.
    ///
    /// Example:
    /// Sacrificial Steel: +220% critical chance bonus (2.2)
    fn critical_chance(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive critical multiplier bonus of the modifier.
    ///
    /// Example:
    /// Organ Shatter: +90% critical multiplier bonus (0.9)
    fn critical_multiplier(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive status chance bonus of the modifier.
    ///
    /// Example:
    /// Melee Prowess: +90% status chance bonus (0.9)
    fn status_chance(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive attack speed bonus of the modifier.
    ///
    /// Example:
    /// Primed Fury: +55% attack speed bonus (0.55)
    ///
    /// Note: This is a melee-specific modifier.
    fn attack_speed(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive fire rate bonus of the modifier.
    ///
    /// Example:
    /// Speed Trigger: +60% fire rate bonus (0.6)
    ///
    /// Note: This is a primary/secondary-specific modifier.
    fn fire_rate(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive status bonus of the modifier.
    ///
    /// Example:
    /// North Wind: +90% cold damage (0.9)
    fn status_list(&self, context: &dyn Weapon) -> Vec<Status>;

    /// Returns the additive ammo maximum bonus of the modifier.
    ///
    /// Example:
    /// Ammo Drum: +90% ammo maximum bonus (0.9)
    ///
    /// Note: This is a primary/secondary-specific modifier.
    fn ammo_maximum(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive magazine capacity bonus of the modifier.
    ///
    /// Example:
    /// Magazine Warp: +30% magazine capacity bonus (0.3)
    ///
    /// Note: This is a primary/secondary-specific modifier.
    fn magazine_capacity(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive multishot bonus of the modifier.
    ///
    /// Example:
    /// Split Chamber: +90% multishot bonus (0.9)
    ///
    /// Note: This is a primary/secondary-specific modifier.
    fn multishot(&self, context: &dyn Weapon) -> f32;

    /// Returns the additive reload speed bonus of the modifier.
    ///
    /// Example:
    /// Fast Hands: +30% reload speed bonus (0.3)
    ///
    /// Note: This is a primary/secondary-specific modifier.
    fn reload_speed(&self, context: &dyn Weapon) -> f32;

    /// Returns the cost of the modifier.
    ///
    /// Example:
    /// Split Chamber: 15
    fn cost(&self, context: &dyn Weapon) -> u8;
}
