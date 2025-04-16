use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedPressurePoint;

#[modifier]
impl Modifier for PrimedPressurePoint {
    fn damage(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.65
    }
}
