mod physical;
mod primary_elemental;
mod secondary_elemental;

pub use physical::Physical;
pub use primary_elemental::PrimaryElemental;
pub use secondary_elemental::SecondaryElemental;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Physical(Physical),
    PrimaryElemental(PrimaryElemental),
    SecondaryElemental(SecondaryElemental),
}

impl Status {
    pub fn damage(&self) -> f32 {
        match self {
            Self::Physical(physical) => physical.damage(),
            Self::PrimaryElemental(elemental) => elemental.damage(),
            Self::SecondaryElemental(secondary) => secondary.damage(),
        }
    }

    pub fn set_damage(&mut self, damage: f32) {
        match self {
            Self::Physical(physical) => physical.set_damage(damage),
            Self::PrimaryElemental(elemental) => elemental.set_damage(damage),
            Self::SecondaryElemental(secondary) => secondary.set_damage(damage),
        }
    }

    pub const fn impact(impact: f32) -> Self {
        Self::Physical(Physical::Impact(impact))
    }

    pub const fn puncture(puncture: f32) -> Self {
        Self::Physical(Physical::Puncture(puncture))
    }

    pub const fn slash(slash: f32) -> Self {
        Self::Physical(Physical::Slash(slash))
    }

    pub const fn cold(cold: f32) -> Self {
        Self::PrimaryElemental(PrimaryElemental::Cold(cold))
    }

    pub const fn electricity(electricity: f32) -> Self {
        Self::PrimaryElemental(PrimaryElemental::Electricity(electricity))
    }

    pub const fn heat(heat: f32) -> Self {
        Self::PrimaryElemental(PrimaryElemental::Heat(heat))
    }

    pub const fn toxin(toxin: f32) -> Self {
        Self::PrimaryElemental(PrimaryElemental::Toxin(toxin))
    }

    pub const fn blast(blast: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Blast(blast))
    }

    pub const fn corrosive(corrosive: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Corrosive(corrosive))
    }

    pub const fn gas(gas: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Gas(gas))
    }

    pub const fn magnetic(magnetic: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Magnetic(magnetic))
    }

    pub const fn radiation(radiation: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Radiation(radiation))
    }

    pub const fn viral(viral: f32) -> Self {
        Self::SecondaryElemental(SecondaryElemental::Viral(viral))
    }
}

pub trait StatusesImpl {
    /// Merge similar statuses into one and elementals into secondary if possible.
    ///
    /// Cold + Electricity = Magnetic
    /// Cold + Heat = Blast
    /// Cold + Toxin = Viral
    /// Electricity + Heat = Radiation
    /// Electricity + Toxin = Corrosive
    /// Heat + Toxin = Gas
    fn merge(&self) -> Vec<Status>;

    /// Calculate the total damage of all statuses.
    fn damage(&self) -> f32;

    fn physical(&self) -> Vec<Status>;
    fn elemental(&self) -> Vec<Status>;
    fn secondary(&self) -> Vec<Status>;

    fn impact(&self) -> Option<Status>;
    fn puncture(&self) -> Option<Status>;
    fn slash(&self) -> Option<Status>;
    fn cold(&self) -> Option<Status>;
    fn electricity(&self) -> Option<Status>;
    fn heat(&self) -> Option<Status>;
    fn toxin(&self) -> Option<Status>;
    fn blast(&self) -> Option<Status>;
    fn corrosive(&self) -> Option<Status>;
    fn gas(&self) -> Option<Status>;
    fn magnetic(&self) -> Option<Status>;
    fn radiation(&self) -> Option<Status>;
    fn viral(&self) -> Option<Status>;
}

impl StatusesImpl for Vec<Status> {
    fn merge(&self) -> Vec<Status> {
        // Merge physicals and elementals into one.
        let mut statuses = self.clone();
        let mut merged_statuses = Vec::new();
        while !statuses.is_empty() {
            let mut status = statuses.remove(0);
            let mut merged_indexes = Vec::new();

            match status {
                Status::Physical(physical) => match physical {
                    Physical::Impact(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Physical(Physical::Impact(impact)) = status_2 {
                                status.set_damage(status.damage() + impact);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Physical::Puncture(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Physical(Physical::Puncture(puncture)) = status_2 {
                                status.set_damage(status.damage() + puncture);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Physical::Slash(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Physical(Physical::Slash(slash)) = status_2 {
                                status.set_damage(status.damage() + slash);
                                merged_indexes.push(index);
                            }
                        }
                    }
                },
                Status::PrimaryElemental(elemental) => match elemental {
                    PrimaryElemental::Cold(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::PrimaryElemental(PrimaryElemental::Cold(cold)) = status_2
                            {
                                status.set_damage(status.damage() + cold);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    PrimaryElemental::Electricity(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::PrimaryElemental(PrimaryElemental::Electricity(
                                electricity,
                            )) = status_2
                            {
                                status.set_damage(status.damage() + electricity);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    PrimaryElemental::Heat(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::PrimaryElemental(PrimaryElemental::Heat(heat)) = status_2
                            {
                                status.set_damage(status.damage() + heat);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    PrimaryElemental::Toxin(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::PrimaryElemental(PrimaryElemental::Toxin(toxin)) =
                                status_2
                            {
                                status.set_damage(status.damage() + toxin);
                                merged_indexes.push(index);
                            }
                        }
                    }
                },
                _ => {}
            }

            for index in merged_indexes.iter().rev() {
                statuses.remove(*index);
            }

            merged_statuses.push(status);
        }

        // Merge elementals into secondary if possible.
        let mut statuses = merged_statuses.clone();
        let mut merged_statuses = Vec::new();
        while !statuses.is_empty() {
            let mut status = statuses.remove(0);

            let index = statuses.iter().position(|status| {
                if let Status::PrimaryElemental(_) = status {
                    true
                } else {
                    false
                }
            });

            match status {
                Status::PrimaryElemental(elemental) => match elemental {
                    PrimaryElemental::Cold(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::PrimaryElemental(elemental_2) => match elemental_2 {
                                    PrimaryElemental::Electricity(electricity) => {
                                        status = Status::magnetic(elemental.damage() + electricity);
                                    }
                                    PrimaryElemental::Heat(heat) => {
                                        status = Status::blast(elemental.damage() + heat);
                                    }
                                    PrimaryElemental::Toxin(toxin) => {
                                        status = Status::viral(elemental.damage() + toxin);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    PrimaryElemental::Electricity(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::PrimaryElemental(elemental_2) => match elemental_2 {
                                    PrimaryElemental::Cold(cold) => {
                                        status = Status::magnetic(elemental.damage() + cold);
                                    }
                                    PrimaryElemental::Heat(heat) => {
                                        status = Status::radiation(elemental.damage() + heat);
                                    }
                                    PrimaryElemental::Toxin(toxin) => {
                                        status = Status::corrosive(elemental.damage() + toxin);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    PrimaryElemental::Heat(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::PrimaryElemental(elemental_2) => match elemental_2 {
                                    PrimaryElemental::Cold(cold) => {
                                        status = Status::blast(elemental.damage() + cold);
                                    }
                                    PrimaryElemental::Electricity(electricity) => {
                                        status =
                                            Status::radiation(elemental.damage() + electricity);
                                    }
                                    PrimaryElemental::Toxin(toxin) => {
                                        status = Status::gas(elemental.damage() + toxin);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    PrimaryElemental::Toxin(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::PrimaryElemental(elemental_2) => match elemental_2 {
                                    PrimaryElemental::Cold(cold) => {
                                        status = Status::viral(elemental.damage() + cold);
                                    }
                                    PrimaryElemental::Electricity(electricity) => {
                                        status =
                                            Status::corrosive(elemental.damage() + electricity);
                                    }
                                    PrimaryElemental::Heat(heat) => {
                                        status = Status::gas(elemental.damage() + heat);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            }

            merged_statuses.push(status);
        }

        // Merge secondaries into one.
        let mut statuses = merged_statuses.clone();
        let mut merged_statuses = Vec::new();
        while !statuses.is_empty() {
            let mut status = statuses.remove(0);
            let mut merged_indexes = Vec::new();

            match status {
                Status::SecondaryElemental(secondary) => match secondary {
                    SecondaryElemental::Blast(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::SecondaryElemental(SecondaryElemental::Blast(blast)) =
                                status_2
                            {
                                status.set_damage(status.damage() + blast);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    SecondaryElemental::Corrosive(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::SecondaryElemental(SecondaryElemental::Corrosive(
                                corrosive,
                            )) = status_2
                            {
                                status.set_damage(status.damage() + corrosive);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    SecondaryElemental::Gas(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::SecondaryElemental(SecondaryElemental::Gas(gas)) =
                                status_2
                            {
                                status.set_damage(status.damage() + gas);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    SecondaryElemental::Magnetic(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::SecondaryElemental(SecondaryElemental::Magnetic(
                                magnetic,
                            )) = status_2
                            {
                                status.set_damage(status.damage() + magnetic);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    SecondaryElemental::Radiation(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::SecondaryElemental(SecondaryElemental::Radiation(
                                radiation,
                            )) = status_2
                            {
                                status.set_damage(status.damage() + radiation);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    SecondaryElemental::Viral(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::SecondaryElemental(SecondaryElemental::Viral(viral)) =
                                status_2
                            {
                                status.set_damage(status.damage() + viral);
                                merged_indexes.push(index);
                            }
                        }
                    }
                },
                _ => {}
            }

            for index in merged_indexes.iter().rev() {
                statuses.remove(*index);
            }

            merged_statuses.push(status);
        }

        merged_statuses
    }

    fn damage(&self) -> f32 {
        self.iter().map(|status| status.damage()).sum::<f32>()
    }

    fn physical(&self) -> Vec<Status> {
        let mut physicals = Vec::new();
        for status in self.iter() {
            if let Status::Physical(_) = status {
                physicals.push(*status);
            }
        }

        physicals
    }

    fn elemental(&self) -> Vec<Status> {
        let mut elementals = Vec::new();
        for status in self.iter() {
            if let Status::PrimaryElemental(_) = status {
                elementals.push(*status);
            }
        }

        elementals
    }

    fn secondary(&self) -> Vec<Status> {
        let mut secondaries = Vec::new();
        for status in self.iter() {
            if let Status::SecondaryElemental(_) = status {
                secondaries.push(*status);
            }
        }

        secondaries
    }

    fn impact(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Physical(Physical::Impact(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn puncture(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Physical(Physical::Puncture(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn slash(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Physical(Physical::Slash(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn cold(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::PrimaryElemental(PrimaryElemental::Cold(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn electricity(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::PrimaryElemental(PrimaryElemental::Electricity(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn heat(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::PrimaryElemental(PrimaryElemental::Heat(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn toxin(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::PrimaryElemental(PrimaryElemental::Toxin(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn blast(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::SecondaryElemental(SecondaryElemental::Blast(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn corrosive(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::SecondaryElemental(SecondaryElemental::Corrosive(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn gas(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::SecondaryElemental(SecondaryElemental::Gas(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn magnetic(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::SecondaryElemental(SecondaryElemental::Magnetic(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn radiation(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::SecondaryElemental(SecondaryElemental::Radiation(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn viral(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::SecondaryElemental(SecondaryElemental::Viral(_)) = status {
                return Some(*status);
            }
        }

        None
    }
}

impl std::ops::Add<f32> for Status {
    type Output = Status;

    fn add(mut self, other: f32) -> Status {
        self.set_damage(self.damage() + other);
        self
    }
}

impl std::ops::AddAssign<f32> for Status {
    fn add_assign(&mut self, other: f32) {
        self.set_damage(self.damage() + other);
    }
}

impl std::ops::Mul<f32> for Status {
    type Output = Status;

    fn mul(mut self, other: f32) -> Status {
        self.set_damage(self.damage() * other);
        self
    }
}

impl std::ops::MulAssign<f32> for Status {
    fn mul_assign(&mut self, other: f32) {
        self.set_damage(self.damage() * other);
    }
}
