use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedPistolGambit;

#[modifier]
impl Modifier for PrimedPistolGambit {
    fn critical_chance(&self, _context: &dyn Weapon) -> f32 {
        1.87
    }
}
