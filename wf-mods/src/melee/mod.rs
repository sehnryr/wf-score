mod berserker_fury;
mod blood_rush;
mod condition_overload;
mod fever_strike;
mod focus_energy;
mod focus_radon;
mod gladiator_might;
mod melee_riven;
mod molten_impact;
mod north_wind;
mod organ_shatter;
mod pressure_point;
mod primed_fever_strike;
mod primed_fury;
mod primed_pressure_point;
mod primed_smite_grineer;
mod sacrificial_steel;
mod shocking_touch;
mod smite_grineer;
mod vicious_frost;
mod virulent_scourge;
mod volcanic_edge;
mod voltaic_strike;
mod weeping_wounds;

pub use berserker_fury::*;
pub use blood_rush::*;
pub use condition_overload::*;
pub use fever_strike::*;
pub use focus_energy::*;
pub use focus_radon::*;
pub use gladiator_might::*;
pub use melee_riven::*;
pub use molten_impact::*;
pub use north_wind::*;
pub use organ_shatter::*;
pub use pressure_point::*;
pub use primed_fever_strike::*;
pub use primed_fury::*;
pub use primed_pressure_point::*;
pub use primed_smite_grineer::*;
pub use sacrificial_steel::*;
pub use shocking_touch::*;
pub use smite_grineer::*;
pub use vicious_frost::*;
pub use virulent_scourge::*;
pub use volcanic_edge::*;
pub use voltaic_strike::*;
pub use weeping_wounds::*;

mod common {
    pub use std::sync::Arc;

    pub use wf_modifier_proc_macro::modifier;
    pub use wf_stats::*;
}

use common::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MeleeMod {
    BerserkerFury,
    BloodRush,

    /// The % of the total damage that a status must have to count as a condition
    ConditionOverload(f32),
    FeverStrike,
    FocusEnergy,
    FocusRadon,
    GladiatorMight,
    MoltenImpact,
    NorthWind,
    OrganShatter,
    PressurePoint,
    PrimedFeverStrike,
    PrimedFury,
    PrimedPressurePoint,
    PrimedSmiteGrineer,
    SacrificialSteel,
    ShockingTouch,
    SmiteGrineer,
    ViciousFrost,
    VirulentScourge,
    VolcanicEdge,
    VoltaicStrike,
    WeepingWounds,

    Riven(MeleeRiven),
}

impl Into<Arc<dyn Modifier>> for MeleeMod {
    fn into(self) -> Arc<dyn Modifier> {
        match self {
            Self::BerserkerFury => Arc::new(BerserkerFury {}),
            Self::BloodRush => Arc::new(BloodRush {}),
            Self::ConditionOverload(threshold) => Arc::new(ConditionOverload { threshold }),
            Self::FeverStrike => Arc::new(FeverStrike {}),
            Self::FocusEnergy => Arc::new(FocusEnergy {}),
            Self::FocusRadon => Arc::new(FocusRadon {}),
            Self::GladiatorMight => Arc::new(GladiatorMight {}),
            Self::MoltenImpact => Arc::new(MoltenImpact {}),
            Self::NorthWind => Arc::new(NorthWind {}),
            Self::OrganShatter => Arc::new(OrganShatter {}),
            Self::PressurePoint => Arc::new(PressurePoint {}),
            Self::PrimedFeverStrike => Arc::new(PrimedFeverStrike {}),
            Self::PrimedFury => Arc::new(PrimedFury {}),
            Self::PrimedPressurePoint => Arc::new(PrimedPressurePoint {}),
            Self::PrimedSmiteGrineer => Arc::new(PrimedSmiteGrineer {}),
            Self::SacrificialSteel => Arc::new(SacrificialSteel {}),
            Self::ShockingTouch => Arc::new(ShockingTouch {}),
            Self::SmiteGrineer => Arc::new(SmiteGrineer {}),
            Self::ViciousFrost => Arc::new(ViciousFrost {}),
            Self::VirulentScourge => Arc::new(VirulentScourge {}),
            Self::VolcanicEdge => Arc::new(VolcanicEdge {}),
            Self::VoltaicStrike => Arc::new(VoltaicStrike {}),
            Self::WeepingWounds => Arc::new(WeepingWounds {}),
            Self::Riven(riven) => Arc::new(riven),
        }
    }
}
