#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

mod melee_riven;
mod mod_combinations;
mod secondary_riven;

use wf_mods::melee::*;
use wf_mods::secondary::*;
use wf_score::melee::*;
use wf_score::secondary::*;
use wf_stats::*;

use crate::melee_riven::generate_melee_riven_combinations;
use crate::mod_combinations::ModCombinations;
use crate::secondary_riven::generate_secondary_riven_combinations;

fn main() {
    let dual_toxocyst = Secondary::new(
        // Incarnon Genesis 4th evolution: Commodore's Fortune
        // +20% base critical chance
        // Default critical chance: 0.05
        0.25,
        2.0,
        37.0,
        1.0,
        1.0,
        72,
        12,
        2.35,
        0.0,
        vec![
            Status::impact(7.5),
            Status::puncture(60.0),
            Status::slash(7.5),
            // Frenzy buff
            Status::toxin(75.0),
        ],
    );

    let dual_toxocyst_riven_disposition = 1.35;

    let riven_mods: Vec<SecondaryMod> = {
        let disposition = dual_toxocyst_riven_disposition;
        let mut riven_mods = Vec::new();
        riven_mods.extend(generate_secondary_riven_combinations(disposition, 2, true));
        riven_mods.extend(generate_secondary_riven_combinations(disposition, 3, true));
        riven_mods
    };

    let status_mods: Vec<SecondaryMod> = vec![
        // Cold
        SecondaryMod::DeepFreeze,
        SecondaryMod::Frostbite,
        SecondaryMod::IceStorm,
        // Electricity
        SecondaryMod::PrimedConvulsion,
        SecondaryMod::Convulsion,
        SecondaryMod::Jolt,
        // Heat
        SecondaryMod::PrimedHeatedCharge,
        SecondaryMod::HeatedCharge,
        SecondaryMod::Scorch,
        // Toxin
        SecondaryMod::PathogenRounds,
        SecondaryMod::PistolPestilence,
        // Radiation
        SecondaryMod::AcceleratedIsotope,
    ];

    let other_mods: Vec<SecondaryMod> = vec![
        // Damage
        SecondaryMod::HornetStrike,
        SecondaryMod::AugurPact,
        // Critical Chance
        SecondaryMod::GalvanizedCrosshairs(5),
        SecondaryMod::CreepingBullseye,
        SecondaryMod::PrimedPistolGambit,
        // Critical Damage
        SecondaryMod::PrimedTargetCracker,
        SecondaryMod::HollowPoint,
        SecondaryMod::SharpenedBullet,
        // Status Chance
        SecondaryMod::GalvanizedShot(0.2),
        // Multishot
        SecondaryMod::GalvanizedDiffusion(4),
        SecondaryMod::LethalTorrent,
        // Fire Rate
        SecondaryMod::AnemicAgility,
        SecondaryMod::Gunslinger,
        // Faction Bonus
        SecondaryMod::PrimedExpelGrineer,
    ];

    let obligatory_mods: Vec<SecondaryMod> = vec![];

    let (best_build, best_score) = bruteforce_secondary(
        dual_toxocyst,
        status_mods,
        other_mods,
        riven_mods,
        obligatory_mods,
        raw_damage,
        48,
        true,
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
/// * `max_cost` - The maximum cost of the build
///
/// # Returns
///
/// A tuple containing the best build and the score relative to the score function
fn bruteforce_secondary(
    secondary: Secondary,
    mut status_mods: Vec<SecondaryMod>,
    mut other_mods: Vec<SecondaryMod>,
    riven_mods: Vec<SecondaryMod>,
    obligatory_mods: Vec<SecondaryMod>,
    score_fn: fn(&Secondary) -> f32,
    max_cost: u8,
    has_reactor: bool,
) -> (Vec<SecondaryMod>, f32) {
    // Remove the duplicate mods
    status_mods.retain(|mod_| !obligatory_mods.contains(mod_));
    other_mods.retain(|mod_| !obligatory_mods.contains(mod_));

    let mut best_build = Vec::new();
    let mut best_score = 0.0;

    for n in 0..=3 {
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

            if weapon.cost(has_reactor) > max_cost {
                continue;
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
/// * `max_cost` - The maximum cost of the build
///
/// # Returns
///
/// A tuple containing the best build and the score relative to the score function
fn bruteforce_melee(
    melee: Melee,
    animation_time: f32,
    combo_hits: f32,
    mut status_mods: Vec<MeleeMod>,
    mut other_mods: Vec<MeleeMod>,
    riven_mods: Vec<MeleeMod>,
    obligatory_mods: Vec<MeleeMod>,
    score_fn: fn(&Melee, f32, f32) -> f32,
    max_cost: u8,
    has_reactor: bool,
) -> (Vec<MeleeMod>, f32) {
    // Remove the duplicate mods
    status_mods.retain(|mod_| !obligatory_mods.contains(mod_));
    other_mods.retain(|mod_| !obligatory_mods.contains(mod_));

    let mut best_build = Vec::new();
    let mut best_score = 0.0;

    for n in 0..=3 {
        let mod_combinations = ModCombinations::new(
            n,
            8,
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

            if weapon.cost(has_reactor) > max_cost {
                continue;
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
