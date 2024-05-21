use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedExpelGrineer;

#[modifier]
impl Modifier for PrimedExpelGrineer {
    fn anti_faction(&self, _context: &dyn Weapon) -> f32 {
        0.55
    }
}
