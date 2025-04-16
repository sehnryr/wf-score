use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LethalTorrent;

#[modifier]
impl Modifier for LethalTorrent {
    fn multishot(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }

    fn fire_rate(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}
