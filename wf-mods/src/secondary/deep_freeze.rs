use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DeepFreeze;

#[modifier]
impl Modifier for DeepFreeze {
    fn status_list(
        &self,
        _context: &dyn Weapon,
    ) -> Vec<Status> {
        vec![Status::cold(0.9)]
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        11
    }
}
