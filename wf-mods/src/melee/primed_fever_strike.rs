use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct PrimedFeverStrike;

#[modifier]
impl Modifier for PrimedFeverStrike {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::toxin(1.65)]
    }
}
