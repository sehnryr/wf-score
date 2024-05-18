use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct SacrificialSteel;

#[modifier]
impl Modifier for SacrificialSteel {
    fn set(&self) -> Option<&str> {
        Some("Sacrificial Steel")
    }

    fn critical_chance(&self, context: &dyn Weapon) -> f32 {
        let mut set_count = 0;
        for modifier in context.modifier_list() {
            if modifier.set() == Some("Sacrificial") {
                set_count += 1;
            }
        }

        let set_bonus = 0.25 * (set_count as f32 - 1.0); // Subtract 1 to account for the current mod

        2.2 * (1.0 + set_bonus)
    }
}
