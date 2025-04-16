use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct VirulentScourge;

#[modifier]
impl Modifier for VirulentScourge {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::toxin(0.6)]
    }

    fn status_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        0.6
    }
}
