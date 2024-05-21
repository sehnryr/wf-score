use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HollowPoint;

#[modifier]
impl Modifier for HollowPoint {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        -0.15
    }

    fn critical_multiplier(&self, _context: &dyn Weapon) -> f32 {
        0.6
    }
}
