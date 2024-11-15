#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use wf_mods::melee::*;
use wf_mods::secondary::*;
use wf_score::melee::*;
use wf_score::secondary::*;
use wf_stats::*;

fn main() {
    let mut dual_toxocyst = Secondary::new(
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

    let riven_mod = SecondaryMod::Riven(SecondaryRiven::new(
        2.703,
        1.994,
        1.227,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        vec![],
    ));

    let build = vec![
        SecondaryMod::PrimedConvulsion,
        riven_mod,
        SecondaryMod::AcceleratedIsotope,
        SecondaryMod::HornetStrike,
        SecondaryMod::PrimedTargetCracker,
        SecondaryMod::GalvanizedDiffusion(4),
        SecondaryMod::GalvanizedCrosshairs(5),
    ];

    for modifier in build {
        dual_toxocyst.add_modifier(modifier.into());
    }

    let score = raw_damage(&dual_toxocyst);

    println!("Score: {}", score);
}
