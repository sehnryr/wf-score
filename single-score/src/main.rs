use wf_mods::melee::*;
use wf_score::melee::melee_influence_dps;
use wf_stats::*;

fn main() {
    let mut ceti_lacera = Melee::new(
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

    let build: [MeleeMod; 8] = [
        MeleeMod::PrimedFeverStrike,
        MeleeMod::NorthWind,
        MeleeMod::ShockingTouch,
        MeleeMod::BerserkerFury,
        // MeleeMod::PrimedSmiteGrineer,
        MeleeMod::OrganShatter,
        MeleeMod::ConditionOverload(0.2),
        MeleeMod::BloodRush(12),
        MeleeMod::WeepingWounds(12),
    ];

    for modifier in build {
        ceti_lacera.add_modifier(modifier.into());
    }

    let score = melee_influence_dps(&ceti_lacera, 4.1, 16.0);

    println!("Score: {}", score);
}
