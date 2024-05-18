use std::sync::Arc;

use crate::status::Status;
use crate::Modifier;

pub trait Weapon {
    /// Returns the damage bonus of the weapon with all modifiers applied.
    fn damage_bonus(&self) -> f32;

    /// Returns the anti-faction multiplier of the weapon with all modifiers applied.
    fn anti_faction(&self) -> f32;

    /// Returns the critical chance of the weapon with all modifiers applied.
    fn critical_chance(&self) -> f32;

    /// Returns the critical multiplier of the weapon with all modifiers applied.
    fn critical_multiplier(&self) -> f32;

    /// Returns the status chance of the weapon with all modifiers applied.
    fn status_chance(&self) -> f32;

    /// Returns the attack speed of the weapon with all modifiers applied.
    fn attack_speed(&self) -> f32;

    /// Returns the final statuses of the weapon.
    ///
    /// Note: This method is used to calculate the final statuses of the weapon when elemental
    /// statuses are to be combined (e.g. Heat + Toxin = Gas).
    fn status_list(&self) -> Vec<Status>;

    /// Returns the modifiers of the weapon.
    fn modifier_list(&self) -> &Vec<Arc<dyn Modifier>>;
}
