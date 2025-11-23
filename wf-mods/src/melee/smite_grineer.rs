use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct SmiteGrineer;

#[modifier]
impl Modifier for SmiteGrineer {
    fn anti_faction(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.3
    }
}
