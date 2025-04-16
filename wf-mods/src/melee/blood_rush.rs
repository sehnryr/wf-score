use super::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BloodRush {
    pub combo_multiplier: u8,
}

#[modifier]
impl Modifier for BloodRush {
    fn critical_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.4 * (self.combo_multiplier - 1).max(1) as f32
    }
}
