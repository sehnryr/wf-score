use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HornetStrike;

#[modifier]
impl Modifier for HornetStrike {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        2.2
    }

    fn cost(&self, _context: &dyn Weapon) -> u8 {
        14
    }
}
