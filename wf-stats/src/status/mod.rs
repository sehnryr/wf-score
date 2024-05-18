mod elemental;
mod physical;
mod secondary;

pub use elemental::Elemental;
pub use physical::Physical;
pub use secondary::Secondary;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Physical(Physical),
    Elemental(Elemental),
    Secondary(Secondary),
}

impl Status {
    pub fn damage(&self) -> f32 {
        match self {
            Self::Physical(physical) => physical.damage(),
            Self::Elemental(elemental) => elemental.damage(),
            Self::Secondary(secondary) => secondary.damage(),
        }
    }

    pub fn set_damage(&mut self, damage: f32) {
        match self {
            Self::Physical(physical) => physical.set_damage(damage),
            Self::Elemental(elemental) => elemental.set_damage(damage),
            Self::Secondary(secondary) => secondary.set_damage(damage),
        }
    }

    pub fn impact(impact: f32) -> Self {
        Self::Physical(Physical::Impact(impact))
    }

    pub fn puncture(puncture: f32) -> Self {
        Self::Physical(Physical::Puncture(puncture))
    }

    pub fn slash(slash: f32) -> Self {
        Self::Physical(Physical::Slash(slash))
    }

    pub fn cold(cold: f32) -> Self {
        Self::Elemental(Elemental::Cold(cold))
    }

    pub fn electricity(electricity: f32) -> Self {
        Self::Elemental(Elemental::Electricity(electricity))
    }

    pub fn heat(heat: f32) -> Self {
        Self::Elemental(Elemental::Heat(heat))
    }

    pub fn toxin(toxin: f32) -> Self {
        Self::Elemental(Elemental::Toxin(toxin))
    }

    pub fn blast(blast: f32) -> Self {
        Self::Secondary(Secondary::Blast(blast))
    }

    pub fn corrosive(corrosive: f32) -> Self {
        Self::Secondary(Secondary::Corrosive(corrosive))
    }

    pub fn gas(gas: f32) -> Self {
        Self::Secondary(Secondary::Gas(gas))
    }

    pub fn magnetic(magnetic: f32) -> Self {
        Self::Secondary(Secondary::Magnetic(magnetic))
    }

    pub fn radiation(radiation: f32) -> Self {
        Self::Secondary(Secondary::Radiation(radiation))
    }

    pub fn viral(viral: f32) -> Self {
        Self::Secondary(Secondary::Viral(viral))
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
                Status::Elemental(elemental) => match elemental {
                    Elemental::Cold(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Elemental(Elemental::Cold(cold)) = status_2 {
                                status.set_damage(status.damage() + cold);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Elemental::Electricity(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Elemental(Elemental::Electricity(electricity)) = status_2
                            {
                                status.set_damage(status.damage() + electricity);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Elemental::Heat(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Elemental(Elemental::Heat(heat)) = status_2 {
                                status.set_damage(status.damage() + heat);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Elemental::Toxin(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Elemental(Elemental::Toxin(toxin)) = status_2 {
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
                if let Status::Elemental(_) = status {
                    true
                } else {
                    false
                }
            });

            match status {
                Status::Elemental(elemental) => match elemental {
                    Elemental::Cold(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::Elemental(elemental_2) => match elemental_2 {
                                    Elemental::Electricity(electricity) => {
                                        status = Status::magnetic(elemental.damage() + electricity);
                                    }
                                    Elemental::Heat(heat) => {
                                        status = Status::blast(elemental.damage() + heat);
                                    }
                                    Elemental::Toxin(toxin) => {
                                        status = Status::viral(elemental.damage() + toxin);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    Elemental::Electricity(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::Elemental(elemental_2) => match elemental_2 {
                                    Elemental::Cold(cold) => {
                                        status = Status::magnetic(elemental.damage() + cold);
                                    }
                                    Elemental::Heat(heat) => {
                                        status = Status::radiation(elemental.damage() + heat);
                                    }
                                    Elemental::Toxin(toxin) => {
                                        status = Status::corrosive(elemental.damage() + toxin);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    Elemental::Heat(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::Elemental(elemental_2) => match elemental_2 {
                                    Elemental::Cold(cold) => {
                                        status = Status::blast(elemental.damage() + cold);
                                    }
                                    Elemental::Electricity(electricity) => {
                                        status =
                                            Status::radiation(elemental.damage() + electricity);
                                    }
                                    Elemental::Toxin(toxin) => {
                                        status = Status::gas(elemental.damage() + toxin);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                    Elemental::Toxin(_) => {
                        if let Some(index) = index {
                            let status_2 = statuses.remove(index);
                            match status_2 {
                                Status::Elemental(elemental_2) => match elemental_2 {
                                    Elemental::Cold(cold) => {
                                        status = Status::viral(elemental.damage() + cold);
                                    }
                                    Elemental::Electricity(electricity) => {
                                        status =
                                            Status::corrosive(elemental.damage() + electricity);
                                    }
                                    Elemental::Heat(heat) => {
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
                Status::Secondary(secondary) => match secondary {
                    Secondary::Blast(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Secondary(Secondary::Blast(blast)) = status_2 {
                                status.set_damage(status.damage() + blast);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Secondary::Corrosive(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Secondary(Secondary::Corrosive(corrosive)) = status_2 {
                                status.set_damage(status.damage() + corrosive);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Secondary::Gas(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Secondary(Secondary::Gas(gas)) = status_2 {
                                status.set_damage(status.damage() + gas);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Secondary::Magnetic(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Secondary(Secondary::Magnetic(magnetic)) = status_2 {
                                status.set_damage(status.damage() + magnetic);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Secondary::Radiation(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Secondary(Secondary::Radiation(radiation)) = status_2 {
                                status.set_damage(status.damage() + radiation);
                                merged_indexes.push(index);
                            }
                        }
                    }
                    Secondary::Viral(_) => {
                        for (index, status_2) in statuses.iter().enumerate() {
                            if let Status::Secondary(Secondary::Viral(viral)) = status_2 {
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
            if let Status::Elemental(_) = status {
                elementals.push(*status);
            }
        }

        elementals
    }

    fn secondary(&self) -> Vec<Status> {
        let mut secondaries = Vec::new();
        for status in self.iter() {
            if let Status::Secondary(_) = status {
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
            if let Status::Elemental(Elemental::Cold(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn electricity(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Elemental(Elemental::Electricity(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn heat(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Elemental(Elemental::Heat(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn toxin(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Elemental(Elemental::Toxin(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn blast(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Secondary(Secondary::Blast(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn corrosive(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Secondary(Secondary::Corrosive(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn gas(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Secondary(Secondary::Gas(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn magnetic(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Secondary(Secondary::Magnetic(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn radiation(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Secondary(Secondary::Radiation(_)) = status {
                return Some(*status);
            }
        }

        None
    }

    fn viral(&self) -> Option<Status> {
        for status in self.iter() {
            if let Status::Secondary(Secondary::Viral(_)) = status {
                return Some(*status);
            }
        }

        None
    }
}
