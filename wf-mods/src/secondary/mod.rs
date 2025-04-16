mod accelerated_isotope;
mod amalgam_barrel_diffusion;
mod anemic_agility;
mod augur_pact;
mod barrel_diffusion;
mod convulsion;
mod creeping_bullseye;
mod deep_freeze;
mod expel_grineer;
mod frostbite;
mod galvanized_crosshairs;
mod galvanized_diffusion;
mod galvanized_shot;
mod gunslinger;
mod heated_charge;
mod hollow_point;
mod hornet_strike;
mod ice_storm;
mod jolt;
mod lethal_torrent;
mod magnum_force;
mod pathogen_rounds;
mod pistol_gambit;
mod pistol_pestilence;
mod primed_convulsion;
mod primed_expel_grineer;
mod primed_heated_charge;
mod primed_pistol_gambit;
mod primed_target_cracker;
mod scorch;
mod secondary_riven;
mod sharpened_bullet;
mod target_cracker;

pub use accelerated_isotope::*;
pub use amalgam_barrel_diffusion::*;
pub use anemic_agility::*;
pub use augur_pact::*;
pub use barrel_diffusion::*;
pub use convulsion::*;
pub use creeping_bullseye::*;
pub use deep_freeze::*;
pub use expel_grineer::*;
pub use frostbite::*;
pub use galvanized_crosshairs::*;
pub use galvanized_diffusion::*;
pub use galvanized_shot::*;
pub use gunslinger::*;
pub use heated_charge::*;
pub use hollow_point::*;
pub use hornet_strike::*;
pub use ice_storm::*;
pub use jolt::*;
pub use lethal_torrent::*;
pub use magnum_force::*;
pub use pathogen_rounds::*;
pub use pistol_gambit::*;
pub use pistol_pestilence::*;
pub use primed_convulsion::*;
pub use primed_expel_grineer::*;
pub use primed_heated_charge::*;
pub use primed_pistol_gambit::*;
pub use primed_target_cracker::*;
pub use scorch::*;
pub use secondary_riven::*;
pub use sharpened_bullet::*;
pub use target_cracker::*;

mod common {
    pub use std::sync::Arc;

    pub use wf_modifier_proc_macro::modifier;
    pub use wf_stats::*;
}

use common::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SecondaryMod {
    AcceleratedIsotope,
    AmalgamBarrelDiffusion,
    CreepingBullseye,
    HollowPoint,
    MagnumForce,
    PrimedConvulsion,
    Scorch,
    AnemicAgility,
    DeepFreeze,
    HornetStrike,
    PrimedHeatedCharge,
    AugurPact,
    Frostbite,
    IceStorm,
    PathogenRounds,
    PrimedPistolGambit,
    SharpenedBullet,
    BarrelDiffusion,
    GalvanizedCrosshairs(u8),
    GalvanizedDiffusion(u8),
    GalvanizedShot(f32),
    Gunslinger,
    Jolt,
    PistolGambit,
    PrimedExpelGrineer,
    ExpelGrineer,
    Convulsion,
    HeatedCharge,
    LethalTorrent,
    PistolPestilence,
    PrimedTargetCracker,
    TargetCracker,
    Riven(SecondaryRiven),
}

impl Into<Arc<dyn Modifier>> for SecondaryMod {
    fn into(self) -> Arc<dyn Modifier> {
        match self {
            Self::AcceleratedIsotope => Arc::new(AcceleratedIsotope {}),
            Self::AmalgamBarrelDiffusion => Arc::new(AmalgamBarrelDiffusion {}),
            Self::CreepingBullseye => Arc::new(CreepingBullseye {}),
            Self::HollowPoint => Arc::new(HollowPoint {}),
            Self::MagnumForce => Arc::new(MagnumForce {}),
            Self::PrimedConvulsion => Arc::new(PrimedConvulsion {}),
            Self::Scorch => Arc::new(Scorch {}),
            Self::AnemicAgility => Arc::new(AnemicAgility {}),
            Self::DeepFreeze => Arc::new(DeepFreeze {}),
            Self::HornetStrike => Arc::new(HornetStrike {}),
            Self::PrimedHeatedCharge => Arc::new(PrimedHeatedCharge {}),
            Self::AugurPact => Arc::new(AugurPact {}),
            Self::Frostbite => Arc::new(Frostbite {}),
            Self::IceStorm => Arc::new(IceStorm {}),
            Self::PathogenRounds => Arc::new(PathogenRounds {}),
            Self::PrimedPistolGambit => Arc::new(PrimedPistolGambit {}),
            Self::SharpenedBullet => Arc::new(SharpenedBullet {}),
            Self::BarrelDiffusion => Arc::new(BarrelDiffusion {}),
            Self::GalvanizedCrosshairs(stacks) => Arc::new(GalvanizedCrosshairs {
                stacks,
            }),
            Self::GalvanizedDiffusion(stacks) => Arc::new(GalvanizedDiffusion {
                stacks,
            }),
            Self::GalvanizedShot(threshold) => Arc::new(GalvanizedShot {
                threshold,
            }),
            Self::Gunslinger => Arc::new(Gunslinger {}),
            Self::Jolt => Arc::new(Jolt {}),
            Self::PistolGambit => Arc::new(PistolGambit {}),
            Self::PrimedExpelGrineer => Arc::new(PrimedExpelGrineer {}),
            Self::ExpelGrineer => Arc::new(ExpelGrineer {}),
            Self::Convulsion => Arc::new(Convulsion {}),
            Self::HeatedCharge => Arc::new(HeatedCharge {}),
            Self::LethalTorrent => Arc::new(LethalTorrent {}),
            Self::PistolPestilence => Arc::new(PistolPestilence {}),
            Self::PrimedTargetCracker => Arc::new(PrimedTargetCracker {}),
            Self::TargetCracker => Arc::new(TargetCracker {}),
            Self::Riven(riven) => Arc::new(riven),
        }
    }
}
