use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Scorch;

#[modifier]
impl Modifier for Scorch {
    fn status_chance(&self, _context: &dyn Weapon) -> f32 {
        0.6
    }

    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::heat(0.6)]
    }
}
