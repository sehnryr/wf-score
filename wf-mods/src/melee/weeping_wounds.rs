use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct WeepingWounds;

#[modifier]
impl Modifier for WeepingWounds {
    fn status_chance(&self, _context: &dyn Weapon) -> f32 {
        4.4
    }
}
