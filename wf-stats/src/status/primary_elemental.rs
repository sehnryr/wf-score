#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimaryElemental {
    Cold(f32),
    Electricity(f32),
    Heat(f32),
    Toxin(f32),
}

impl PrimaryElemental {
    pub fn damage(&self) -> f32 {
        match self {
            Self::Cold(cold) => *cold,
            Self::Electricity(electricity) => *electricity,
            Self::Heat(heat) => *heat,
            Self::Toxin(toxin) => *toxin,
        }
    }

    pub fn set_damage(
        &mut self,
        damage: f32,
    ) {
        match self {
            Self::Cold(cold) => *cold = damage,
            Self::Electricity(electricity) => *electricity = damage,
            Self::Heat(heat) => *heat = damage,
            Self::Toxin(toxin) => *toxin = damage,
        }
    }
}
