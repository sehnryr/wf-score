use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct IceStorm;

#[modifier]
impl Modifier for IceStorm {
    fn magazine_capacity(&self, _context: &dyn Weapon) -> f32 {
        0.4
    }

    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        vec![Status::cold(0.4)]
    }
}
