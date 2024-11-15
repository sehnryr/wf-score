use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AnemicAgility;

#[modifier]
impl Modifier for AnemicAgility {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        -0.15
    }

    fn fire_rate(&self, _context: &dyn Weapon) -> f32 {
        0.9
    }

    fn cost(&self, _context: &dyn Weapon) -> u8 {
        9
    }
}
