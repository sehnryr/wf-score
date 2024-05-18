use super::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BloodRush;

#[modifier]
impl Modifier for BloodRush {
    fn critical_chance(&self, _context: &dyn Weapon) -> f32 {
        4.4
    }
}
