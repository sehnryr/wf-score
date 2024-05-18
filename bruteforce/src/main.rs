mod mod_combinations;
mod riven;

use wf_mods::melee::*;
use wf_score::melee_influence_dps;
use wf_stats::*;

use crate::mod_combinations::ModCombinations;
use crate::riven::generate_riven_combinations;

fn main() {
    let ceti_lacera = Melee::new(
        0.2,
        2.0,
        0.4,
        1.08,
        vec![
            Status::impact(12.0),
            Status::puncture(38.0),
            Status::slash(66.0),
            Status::electricity(100.0),
        ],
    );

    // Defiled Snapdragon Stance mod
    let animation_time = 4.1;
    let combo_hits = 16.0;

    let ceti_lacera_riven_disposition = 1.20;

    let riven_mods = {
        let disposition = ceti_lacera_riven_disposition;
        let mut riven_mods = Vec::new();
        riven_mods.extend(generate_riven_combinations(disposition, 2, 1.2375));
        riven_mods.extend(generate_riven_combinations(disposition, 3, 0.9375));
        riven_mods
    };

    let status_mods = vec![
        MeleeMod::NorthWind,
        MeleeMod::ShockingTouch,
        MeleeMod::MoltenImpact,
        MeleeMod::FeverStrike,
        MeleeMod::PrimedFeverStrike,
        MeleeMod::ViciousFrost,
        MeleeMod::VoltaicStrike,
        MeleeMod::VolcanicEdge,
        MeleeMod::VirulentScourge,
        MeleeMod::FocusEnergy,
        MeleeMod::FocusRadon,
    ];

    let other_mods = vec![
        MeleeMod::GladiatorMight,
        MeleeMod::SacrificialSteel,
        MeleeMod::OrganShatter,
        MeleeMod::ConditionOverload(0.2),
    ];

    let obligatory_mods = vec![
        MeleeMod::BerserkerFury,
        MeleeMod::BloodRush,
        MeleeMod::WeepingWounds,
    ];

    let (best_build, best_score) = bruteforce(
        ceti_lacera,
        animation_time,
        combo_hits,
        status_mods,
        other_mods,
        riven_mods,
        obligatory_mods,
        melee_influence_dps,
    );

    println!("Best build: {:?}", best_build);
    println!("Score: {}", best_score);
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
fn bruteforce(
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
        let mod_combinations =
            ModCombinations::new(n, &status_mods, &other_mods, &riven_mods, &obligatory_mods);

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
