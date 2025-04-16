use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ShockingTouch;

#[modifier]
impl Modifier for ShockingTouch {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::electricity(0.9)]
    }
}
