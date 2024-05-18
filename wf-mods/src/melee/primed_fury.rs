use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedFury;

#[modifier]
impl Modifier for PrimedFury {
    fn attack_speed(&self, _context: &dyn Weapon) -> f32 {
        0.55
    }
}
