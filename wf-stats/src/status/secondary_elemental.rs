#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecondaryElemental {
    Blast(f32),
    Corrosive(f32),
    Gas(f32),
    Magnetic(f32),
    Radiation(f32),
    Viral(f32),
}

impl SecondaryElemental {
    pub fn damage(&self) -> f32 {
        match self {
            Self::Blast(blast) => *blast,
            Self::Corrosive(corrosive) => *corrosive,
            Self::Gas(gas) => *gas,
            Self::Magnetic(magnetic) => *magnetic,
            Self::Radiation(radiation) => *radiation,
            Self::Viral(viral) => *viral,
        }
    }

    pub fn set_damage(&mut self, damage: f32) {
        match self {
            Self::Blast(blast) => *blast = damage,
            Self::Corrosive(corrosive) => *corrosive = damage,
            Self::Gas(gas) => *gas = damage,
            Self::Magnetic(magnetic) => *magnetic = damage,
            Self::Radiation(radiation) => *radiation = damage,
            Self::Viral(viral) => *viral = damage,
        }
    }
}
