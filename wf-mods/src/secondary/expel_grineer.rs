use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ExpelGrineer;

#[modifier]
impl Modifier for ExpelGrineer {
    fn anti_faction(&self, _context: &dyn Weapon) -> f32 {
        0.3
    }
}
