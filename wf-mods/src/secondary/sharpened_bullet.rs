use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SharpenedBullet;

#[modifier]
impl Modifier for SharpenedBullet {
    fn critical_multiplier(&self, _context: &dyn Weapon) -> f32 {
        0.75
    }
}
