use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PrimedHeatedCharge;

#[modifier]
impl Modifier for PrimedHeatedCharge {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::heat(1.65)]
    }
}
