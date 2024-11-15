use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Jolt;

#[modifier]
impl Modifier for Jolt {
    fn status_chance(&self, _context: &dyn Weapon) -> f32 {
        0.6
    }

    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::electricity(0.6)]
    }

    fn cost(&self, _context: &dyn Weapon) -> u8 {
        7
    }
}
