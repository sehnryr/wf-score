use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct NorthWind;

#[modifier]
impl Modifier for NorthWind {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::cold(0.9)]
    }
}
