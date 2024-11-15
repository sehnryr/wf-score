use wf_stats::*;

pub fn raw_damage(secondary: &Secondary) -> f32 {
    let damage_bonus = secondary.damage_bonus();
    let critical_chance = secondary.critical_chance();
    let critical_multiplier = secondary.critical_multiplier();
    let anti_faction = secondary.anti_faction();
    let fire_rate = secondary.fire_rate();
    let magazine_capacity = secondary.magazine_capacity();
    let multishot = secondary.multishot();
    let reload_speed = secondary.reload_speed();
    let reload_delay = secondary.reload_delay();
    let _status_chance = secondary.status_chance();
    let status_list = secondary.status_list();
    let total_damage = status_list.damage();

    let secondary_dph = total_damage
        * (1.0 + damage_bonus)
        * (1.0 + critical_chance * (critical_multiplier - 1.0))
        * (1.0 + anti_faction)
        * multishot;

    let time_magazine = magazine_capacity as f32 / fire_rate;
    let time_total = time_magazine + reload_speed + reload_delay;
    let fire_rate_avg = magazine_capacity as f32 / time_total;

    let secondary_dps = secondary_dph * fire_rate_avg;

    secondary_dps
}
