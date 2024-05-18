use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FocusEnergy;

#[modifier]
impl Modifier for FocusEnergy {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::electricity(0.6)]
    }
}
