use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct OrganShatter;

#[modifier]
impl Modifier for OrganShatter {
    fn critical_multiplier(&self, _context: &dyn Weapon) -> f32 {
        0.9
    }
}
