use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PressurePoint;

#[modifier]
impl Modifier for PressurePoint {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        1.2
    }
}
