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

macro_rules! damage {
    ($d:expr) => {
        $d
    };
    () => {
        _
    };
}

macro_rules! variant {
    (Physical) => { Status::Physical(_) };
    (PrimaryElemental) => { Status::PrimaryElemental(_) };
    (SecondaryElemental) => { Status::SecondaryElemental(_) };
    // Physical
    (Impact $(,$d:expr)?) => { Status::Physical(Physical::Impact(damage!($($d)?))) };
    (Puncture $(,$d:expr)?) => { Status::Physical(Physical::Puncture(damage!($($d)?))) };
    (Slash $(,$d:expr)?) => { Status::Physical(Physical::Slash(damage!($($d)?))) };
    // Primary Elemental
    (Cold $(,$d:expr)?) => { Status::PrimaryElemental(PrimaryElemental::Cold(damage!($($d)?))) };
    (Electricity $(,$d:expr)?) => { Status::PrimaryElemental(PrimaryElemental::Electricity(damage!($($d)?))) };
    (Heat $(,$d:expr)?) => { Status::PrimaryElemental(PrimaryElemental::Heat(damage!($($d)?))) };
    (Toxin $(,$d:expr)?) => { Status::PrimaryElemental(PrimaryElemental::Toxin(damage!($($d)?))) };
    // Secondary Elemental
    (Blast $(,$d:expr)?) => { Status::SecondaryElemental(SecondaryElemental::Blast(damage!($($d)?))) };
    (Corrosive $(,$d:expr)?) => { Status::SecondaryElemental(SecondaryElemental::Corrosive(damage!($($d)?))) };
    (Gas $(,$d:expr)?) => { Status::SecondaryElemental(SecondaryElemental::Gas(damage!($($d)?))) };
    (Magnetic $(,$d:expr)?) => { Status::SecondaryElemental(SecondaryElemental::Magnetic(damage!($($d)?))) };
    (Radiation $(,$d:expr)?) => { Status::SecondaryElemental(SecondaryElemental::Radiation(damage!($($d)?))) };
    (Viral $(,$d:expr)?) => { Status::SecondaryElemental(SecondaryElemental::Viral(damage!($($d)?))) };
}

macro_rules! is_status {
    ($($is_status:ident ( $variant:ident ) ),+ $(,)?) => {
        $(
            pub const fn $is_status(&self) -> bool { matches!(*self, variant!($variant)) }
        )+
    };
}

macro_rules! set_status {
    ($($set_status:ident ( $variant:ident ) ),+ $(,)?) => {
        $(
            pub const fn $set_status(damage: f32) -> Self { variant!($variant, damage) }
        )+
    };
}

impl Status {
    is_status!(
        is_physical(Physical),
        is_primary_elemental(PrimaryElemental),
        is_secondary_elemental(SecondaryElemental),
        // Physical
        is_impact(Impact),
        is_puncture(Puncture),
        is_slash(Slash),
        // Primary Elemental
        is_cold(Cold),
        is_electricity(Electricity),
        is_heat(Heat),
        is_toxin(Toxin),
        // Secondary Elemental
        is_blast(Blast),
        is_corrosive(Corrosive),
        is_gas(Gas),
        is_magnetic(Magnetic),
        is_radiation(Radiation),
        is_viral(Viral),
    );

    set_status!(
        // Physical
        impact(Impact),
        puncture(Puncture),
        slash(Slash),
        // Primary Elemental
        cold(Cold),
        electricity(Electricity),
        heat(Heat),
        toxin(Toxin),
        // Secondary Elemental
        blast(Blast),
        corrosive(Corrosive),
        gas(Gas),
        magnetic(Magnetic),
        radiation(Radiation),
        viral(Viral),
    );

    pub fn damage(&self) -> f32 {
        match self {
            Self::Physical(physical) => physical.damage(),
            Self::PrimaryElemental(elemental) => elemental.damage(),
            Self::SecondaryElemental(secondary) => secondary.damage(),
        }
    }

    pub fn set_damage(
        &mut self,
        damage: f32,
    ) {
        match self {
            Self::Physical(physical) => physical.set_damage(damage),
            Self::PrimaryElemental(elemental) => elemental.set_damage(damage),
            Self::SecondaryElemental(secondary) => secondary.set_damage(damage),
        }
    }
}

pub trait StatusesImpl {
    /// Merge similar statuses into one and elementals into secondary if
    /// possible.
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
                Status::SecondaryElemental(_) => {}
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

            let index = statuses.iter().position(|status| status.is_physical());

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
                                    PrimaryElemental::Cold(_) => {}
                                },
                                Status::Physical(_) | Status::SecondaryElemental(_) => {}
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
                                    PrimaryElemental::Electricity(_) => {}
                                },
                                Status::Physical(_) | Status::SecondaryElemental(_) => {}
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
                                    PrimaryElemental::Heat(_) => {}
                                },
                                Status::Physical(_) | Status::SecondaryElemental(_) => {}
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
                                    PrimaryElemental::Toxin(_) => {}
                                },
                                Status::Physical(_) | Status::SecondaryElemental(_) => {}
                            }
                        }
                    }
                },
                Status::Physical(_) | Status::SecondaryElemental(_) => {}
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
                Status::Physical(_) | Status::PrimaryElemental(_) => {}
            }

            for index in merged_indexes.iter().rev() {
                statuses.remove(*index);
            }

            merged_statuses.push(status);
        }

        merged_statuses
    }

    fn damage(&self) -> f32 { self.iter().map(|status| status.damage()).sum::<f32>() }

    fn physical(&self) -> Vec<Status> {
        let mut physicals = Vec::new();
        for status in self.iter() {
            if status.is_physical() {
                physicals.push(*status);
            }
        }

        physicals
    }

    fn elemental(&self) -> Vec<Status> {
        let mut elementals = Vec::new();
        for status in self.iter() {
            if status.is_primary_elemental() {
                elementals.push(*status);
            }
        }

        elementals
    }

    fn secondary(&self) -> Vec<Status> {
        let mut secondaries = Vec::new();
        for status in self.iter() {
            if status.is_secondary_elemental() {
                secondaries.push(*status);
            }
        }

        secondaries
    }

    fn impact(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_impact() {
                return Some(*status);
            }
        }

        None
    }

    fn puncture(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_puncture() {
                return Some(*status);
            }
        }

        None
    }

    fn slash(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_slash() {
                return Some(*status);
            }
        }

        None
    }

    fn cold(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_cold() {
                return Some(*status);
            }
        }

        None
    }

    fn electricity(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_electricity() {
                return Some(*status);
            }
        }

        None
    }

    fn heat(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_heat() {
                return Some(*status);
            }
        }

        None
    }

    fn toxin(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_toxin() {
                return Some(*status);
            }
        }

        None
    }

    fn blast(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_blast() {
                return Some(*status);
            }
        }

        None
    }

    fn corrosive(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_corrosive() {
                return Some(*status);
            }
        }

        None
    }

    fn gas(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_gas() {
                return Some(*status);
            }
        }

        None
    }

    fn magnetic(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_magnetic() {
                return Some(*status);
            }
        }

        None
    }

    fn radiation(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_radiation() {
                return Some(*status);
            }
        }

        None
    }

    fn viral(&self) -> Option<Status> {
        for status in self.iter() {
            if status.is_viral() {
                return Some(*status);
            }
        }

        None
    }
}

impl std::ops::Add<f32> for Status {
    type Output = Status;

    fn add(
        mut self,
        other: f32,
    ) -> Status {
        self.set_damage(self.damage() + other);
        self
    }
}

impl std::ops::AddAssign<f32> for Status {
    fn add_assign(
        &mut self,
        other: f32,
    ) {
        self.set_damage(self.damage() + other);
    }
}

impl std::ops::Mul<f32> for Status {
    type Output = Status;

    fn mul(
        mut self,
        other: f32,
    ) -> Status {
        self.set_damage(self.damage() * other);
        self
    }
}

impl std::ops::MulAssign<f32> for Status {
    fn mul_assign(
        &mut self,
        other: f32,
    ) {
        self.set_damage(self.damage() * other);
    }
}
