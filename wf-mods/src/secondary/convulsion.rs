use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Convulsion;

#[modifier]
impl Modifier for Convulsion {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::electricity(0.9)]
    }
}
