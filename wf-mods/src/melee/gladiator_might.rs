use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct GladiatorMight {
    pub combo_multiplier: u8,
}

#[modifier]
impl Modifier for GladiatorMight {
    fn set(&self) -> Option<&str> {
        Some("Gladiator")
    }

    fn critical_chance(&self, context: &dyn Weapon) -> f32 {
        let mut set_count = 0;
        for modifier in context.modifier_list() {
            if modifier.set() == Some("Gladiator") {
                set_count += 1;
            }
        }

        let set_bonus = 1.0 * (set_count as f32 - 1.0); // Subtract 1 to account for the current mod

        0.1 * (self.combo_multiplier - 1).max(1) as f32 * (1.0 + set_bonus)
    }

    fn critical_multiplier(&self, _context: &dyn Weapon) -> f32 {
        0.6
    }
}
