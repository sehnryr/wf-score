use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PathogenRounds;

#[modifier]
impl Modifier for PathogenRounds {
    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::toxin(0.9)]
    }
}
