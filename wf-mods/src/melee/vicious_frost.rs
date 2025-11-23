use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ViciousFrost;

#[modifier]
impl Modifier for ViciousFrost {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::cold(0.6)]
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}
