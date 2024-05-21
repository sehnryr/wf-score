use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Gunslinger;

#[modifier]
impl Modifier for Gunslinger {
    fn fire_rate(&self, _context: &dyn Weapon) -> f32 {
        0.72
    }
}
