use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CreepingBullseye;

#[modifier]
impl Modifier for CreepingBullseye {
    fn critical_chance(&self, _context: &dyn Weapon) -> f32 {
        2.0
    }

    fn fire_rate(&self, _context: &dyn Weapon) -> f32 {
        -0.2
    }
}
