#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Physical {
    Impact(f32),
    Puncture(f32),
    Slash(f32),
}

impl Physical {
    pub fn damage(&self) -> f32 {
        match self {
            Self::Impact(impact) => *impact,
            Self::Puncture(puncture) => *puncture,
            Self::Slash(slash) => *slash,
        }
    }

    pub fn set_damage(
        &mut self,
        damage: f32,
    ) {
        match self {
            Self::Impact(impact) => *impact = damage,
            Self::Puncture(puncture) => *puncture = damage,
            Self::Slash(slash) => *slash = damage,
        }
    }
}
