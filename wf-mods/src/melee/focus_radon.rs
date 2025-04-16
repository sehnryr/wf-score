use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct FocusRadon;

#[modifier]
impl Modifier for FocusRadon {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::radiation(0.6)]
    }
}
