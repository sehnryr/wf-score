use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct MoltenImpact;

#[modifier]
impl Modifier for MoltenImpact {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::heat(0.9)]
    }
}
