mod melee_riven;
mod mod_combinations;
mod secondary_riven;

use wf_mods::melee::*;
use wf_mods::secondary::*;
use wf_score::melee::{melee_influence_dph, melee_influence_dps};
use wf_score::secondary::cascadia_flare_dps;
use wf_stats::*;

use crate::melee_riven::generate_melee_riven_combinations;
use crate::mod_combinations::ModCombinations;
use crate::secondary_riven::generate_secondary_riven_combinations;

fn main() {
    let tenet_cycron = Secondary::new(
        0.2,
        1.8,
        0.4,
        12.0,
        1.0,
        0,
        40,
        1.5,
        0.5,
        vec![
            Status::heat(22.0),
            Status::radiation(22.0 * 0.6), // Sister Element
        ],
    );

    let tenet_cycron_riven_disposition = 0.70;

    let riven_mods: Vec<SecondaryMod> = {
        let disposition = tenet_cycron_riven_disposition;
        let mut riven_mods = Vec::new();
        riven_mods.extend(generate_secondary_riven_combinations(
            disposition,
            2,
            1.2375,
        ));
        riven_mods.extend(generate_secondary_riven_combinations(
            disposition,
            3,
            0.9375,
        ));
        riven_mods
    };

    // let riven_mods = {
    //     let disposition = tenet_cycron_riven_disposition;
    //     let mut riven_mods = Vec::new();
    //     riven_mods.extend(generate_melee_riven_combinations(disposition, 2, 1.2375));
    //     riven_mods.extend(generate_melee_riven_combinations(disposition, 3, 0.9375));
    //     riven_mods
    // };

    let status_mods: Vec<SecondaryMod> = vec![
        // Cold
        SecondaryMod::DeepFreeze,
        SecondaryMod::Frostbite,
        SecondaryMod::IceStorm,
        // Electricity
        // SecondaryMod::PrimedConvulsion,
        // SecondaryMod::Convulsion,
        // SecondaryMod::Jolt,
        // Heat
        // SecondaryMod::PrimedHeatedCharge,
        // SecondaryMod::HeatedCharge,
        // SecondaryMod::Scorch,
        // Toxin
        SecondaryMod::PathogenRounds,
        SecondaryMod::PistolPestilence,
        // Radiation
        // SecondaryMod::AcceleratedIsotope,
    ];

    let other_mods: Vec<SecondaryMod> = vec![
        // Damage
        SecondaryMod::HornetStrike,
        SecondaryMod::AugurPact,
        // Critical Chance
        SecondaryMod::CreepingBullseye,
        SecondaryMod::PrimedPistolGambit,
        // SecondaryMod::PistolGambit,
        // Critical Damage
        SecondaryMod::PrimedTargetCracker,
        // SecondaryMod::TargetCracker,
        SecondaryMod::HollowPoint,
        SecondaryMod::SharpenedBullet,
        // Multishot
        SecondaryMod::GalvanizedDiffusion(4),
        // SecondaryMod::BarrelDiffusion,
        SecondaryMod::LethalTorrent,
        // Fire Rate
        SecondaryMod::AnemicAgility,
        SecondaryMod::Gunslinger,
    ];

    let obligatory_mods: Vec<SecondaryMod> = vec![
        // MeleeMod::BerserkerFury,
        // MeleeMod::BloodRush(12),
        // MeleeMod::WeepingWounds(12),
        // MeleeMod::ConditionOverload(0.2),
        SecondaryMod::PrimedHeatedCharge,
        // SecondaryMod::PistolPestilence,
        // SecondaryMod::PrimedPistolGambit,
        // SecondaryMod::PrimedTargetCracker,
        // SecondaryMod::GalvanizedDiffusion(4),
        // SecondaryMod::LethalTorrent,
    ];

    let (best_build, best_score) = bruteforce_secondary(
        tenet_cycron,
        status_mods,
        other_mods,
        riven_mods,
        obligatory_mods,
        cascadia_flare_dps,
    );

    println!("Best build: {:?}", best_build);
    println!("Score: {}", best_score);
}

/// Brute force the best build for a secondary weapon
///
/// # Arguments
///
/// * `secondary` - The secondary weapon
/// * `status_mods` - The list of status mods
/// * `other_mods` - The list of other mods
/// * `riven_mods` - The list of riven mods
/// * `obligatory_mods` - The list of obligatory mods
/// * `score_fn` - The function to calculate the score
///
/// # Returns
///
/// A tuple containing the best build and the score relative to the score function
fn bruteforce_secondary(
    secondary: Secondary,
    status_mods: Vec<SecondaryMod>,
    other_mods: Vec<SecondaryMod>,
    riven_mods: Vec<SecondaryMod>,
    obligatory_mods: Vec<SecondaryMod>,
    score_fn: fn(&Secondary) -> f32,
) -> (Vec<SecondaryMod>, f32) {
    let mut best_build = Vec::new();
    let mut best_score = 0.0;

    for n in 1..=3 {
        let mod_combinations = ModCombinations::new(
            n,
            8,
            &status_mods,
            &other_mods,
            &riven_mods,
            &obligatory_mods,
        );

        for build in mod_combinations {
            let mut weapon = secondary.clone();

            for modifier in build.iter() {
                weapon.add_modifier(modifier.clone().into());
            }

            let score = score_fn(&weapon);

            if score > best_score {
                best_build = build.clone();
                best_score = score;
            }
        }

        println!("Score: {}", best_score);
    }

    (best_build, best_score)
}

/// Brute force the best build for a melee weapon
///
/// # Arguments
///
/// * `melee` - The melee weapon
/// * `animation_time` - The animation time of the combo from the stance mod
/// * `combo_hits` - The number of hits in the combo
/// * `status_mods` - The list of status mods
/// * `other_mods` - The list of other mods
/// * `riven_mods` - The list of riven mods
/// * `obligatory_mods` - The list of obligatory mods
/// * `score_fn` - The function to calculate the score
///
/// # Returns
///
/// A tuple containing the best build and the score relative to the score function
fn bruteforce_melee(
    melee: Melee,
    animation_time: f32,
    combo_hits: f32,
    status_mods: Vec<MeleeMod>,
    other_mods: Vec<MeleeMod>,
    riven_mods: Vec<MeleeMod>,
    obligatory_mods: Vec<MeleeMod>,
    score_fn: fn(&Melee, f32, f32) -> f32,
) -> (Vec<MeleeMod>, f32) {
    let mut best_build = Vec::new();
    let mut best_score = 0.0;

    for n in 1..=3 {
        let mod_combinations = ModCombinations::new(
            n,
            7,
            &status_mods,
            &other_mods,
            &riven_mods,
            &obligatory_mods,
        );

        for build in mod_combinations {
            let mut weapon = melee.clone();

            for modifier in build.iter() {
                weapon.add_modifier(modifier.clone().into());
            }

            let score = score_fn(&weapon, animation_time, combo_hits);

            if score > best_score {
                best_build = build.clone();
                best_score = score;
            }
        }

        println!("Score: {}", best_score);
    }

    (best_build, best_score)
}
