use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedConvulsion;

#[modifier]
impl Modifier for PrimedConvulsion {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::electricity(1.65)]
    }
}
