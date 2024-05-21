use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TargetCracker;

#[modifier]
impl Modifier for TargetCracker {
    fn critical_multiplier(&self, _context: &dyn Weapon) -> f32 {
        0.6
    }
}
