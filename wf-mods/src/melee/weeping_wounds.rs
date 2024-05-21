use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct WeepingWounds {
    pub combo_multiplier: u8,
}

#[modifier]
impl Modifier for WeepingWounds {
    fn status_chance(&self, _context: &dyn Weapon) -> f32 {
        0.4 * (self.combo_multiplier - 1).max(1) as f32
    }
}
