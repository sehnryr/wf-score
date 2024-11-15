use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AmalgamBarrelDiffusion;

#[modifier]
impl Modifier for AmalgamBarrelDiffusion {
    fn multishot(&self, _context: &dyn Weapon) -> f32 {
        1.1
    }

    fn cost(&self, _context: &dyn Weapon) -> u8 {
        15
    }
}
