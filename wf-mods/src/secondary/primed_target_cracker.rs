use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedTargetCracker;

#[modifier]
impl Modifier for PrimedTargetCracker {
    fn critical_multiplier(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.1
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        14
    }
}
