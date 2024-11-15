use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AugurPact;

#[modifier]
impl Modifier for AugurPact {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        0.9
    }

    fn cost(&self, _context: &dyn Weapon) -> u8 {
        7
    }
}
