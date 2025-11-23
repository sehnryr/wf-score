use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedSmiteGrineer;

#[modifier]
impl Modifier for PrimedSmiteGrineer {
    fn anti_faction(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.55
    }
}
