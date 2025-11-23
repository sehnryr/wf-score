use super::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BerserkerFury;

#[modifier]
impl Modifier for BerserkerFury {
    fn attack_speed(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.7
    }
}
