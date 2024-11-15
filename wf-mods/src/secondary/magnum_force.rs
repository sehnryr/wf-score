use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MagnumForce;

#[modifier]
impl Modifier for MagnumForce {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        1.65
    }

    // -0.55 accuracy

    fn cost(&self, _context: &dyn Weapon) -> u8 {
        14
    }
}
