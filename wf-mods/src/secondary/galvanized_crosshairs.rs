use super::common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GalvanizedCrosshairs {
    pub stacks: u8,
}

#[modifier]
impl Modifier for GalvanizedCrosshairs {
    // When aiming, critical chance is increased by 120% + 40% per headshot kills
    // (up to 5 stacks)
    fn critical_chance(
        &self,
        _context: &dyn Weapon,
    ) -> f32 {
        1.2 + 0.4 * self.stacks.min(5) as f32
    }

    fn cost(
        &self,
        _context: &dyn Weapon,
    ) -> u8 {
        12
    }
}
