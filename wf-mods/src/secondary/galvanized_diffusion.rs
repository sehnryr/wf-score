use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GalvanizedDiffusion {
    pub stacks: u8,
}

#[modifier]
impl Modifier for GalvanizedDiffusion {
    fn multishot(&self, _context: &dyn Weapon) -> f32 {
        1.1 + 0.3 * self.stacks.min(4) as f32
    }
}
