use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct VoltaicStrike;

#[modifier]
impl Modifier for VoltaicStrike {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::electricity(0.6)]
    }

    fn status_chance(&self, _context: &dyn Weapon) -> f32 {
        0.6
    }
}
