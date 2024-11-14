use std::sync::Arc;

use derivative::Derivative;

use crate::modifier::{Modifier, WeaponModifiers};
use crate::status::{Physical, Status, StatusesImpl};
use crate::weapon::Weapon;

#[derive(Derivative, Default, Clone)]
#[derivative(Debug)]
pub struct Secondary {
    critical_chance: f32,
    critical_multiplier: f32,
    status_chance: f32,
    fire_rate: f32,
    multishot: f32,
    ammo_maximum: usize,
    magazine_size: usize,
    reload_time: f32,
    reload_delay: f32,
    status_list: Vec<Status>,
    #[derivative(Debug = "ignore")]
    modifier_list: Vec<Arc<dyn Modifier>>,
}

impl Secondary {
    pub fn new(
        critical_chance: f32,
        critical_multiplier: f32,
        status_chance: f32,
        fire_rate: f32,
        multishot: f32,
        ammo_maximum: usize,
        magazine_size: usize,
        reload_time: f32,
        reload_delay: f32,
        status_list: Vec<Status>,
    ) -> Self {
        Self {
            critical_chance,
            critical_multiplier,
            status_chance,
            fire_rate,
            multishot,
            ammo_maximum,
            magazine_size,
            reload_time,
            reload_delay,
            status_list,
            modifier_list: Vec::new(),
        }
    }

    fn base_damage(&self) -> f32 {
        self.status_list.iter().map(|status| status.damage()).sum()
    }
}

impl WeaponModifiers for Secondary {
    fn add_modifier(&mut self, modifier: Arc<dyn Modifier>) {
        self.modifier_list.push(modifier);
    }
}

impl Weapon for Secondary
where
    Self: Default,
{
    fn attack_speed(&self) -> f32 {
        Default::default()
    }

    fn damage_bonus(&self) -> f32 {
        let mut damage_bonus = 0.0;
        for modifier in self.modifier_list.iter() {
            damage_bonus += modifier.damage(self);
        }
        damage_bonus
    }

    fn anti_faction(&self) -> f32 {
        let mut anti_faction = 0.0;
        for modifier in self.modifier_list.iter() {
            let modifier_anti_faction = modifier.anti_faction(self);
            if modifier_anti_faction > anti_faction {
                anti_faction = modifier_anti_faction;
            }
        }
        anti_faction
    }

    fn critical_chance(&self) -> f32 {
        let mut critical_chance = 0.0;
        for modifier in self.modifier_list.iter() {
            critical_chance += modifier.critical_chance(self);
        }
        self.critical_chance * (1.0 + critical_chance)
    }

    fn critical_multiplier(&self) -> f32 {
        let mut critical_multiplier = 0.0;
        for modifier in self.modifier_list.iter() {
            critical_multiplier += modifier.critical_multiplier(self);
        }
        self.critical_multiplier * (1.0 + critical_multiplier)
    }

    fn status_chance(&self) -> f32 {
        let mut status_chance = 0.0;
        for modifier in self.modifier_list.iter() {
            status_chance += modifier.status_chance(self);
        }
        self.status_chance * (1.0 + status_chance)
    }

    fn fire_rate(&self) -> f32 {
        let mut fire_rate = 0.0;
        for modifier in self.modifier_list.iter() {
            fire_rate += modifier.fire_rate(self);
        }
        self.fire_rate * (1.0 + fire_rate)
    }

    fn ammo_maximum(&self) -> usize {
        let mut ammo_maximum = 0.0;
        for modifier in self.modifier_list.iter() {
            ammo_maximum += modifier.ammo_maximum(self);
        }
        (self.ammo_maximum as f32 * (1.0 + ammo_maximum)) as usize
    }

    fn magazine_capacity(&self) -> usize {
        let mut magazine_capacity = 0.0;
        for modifier in self.modifier_list.iter() {
            magazine_capacity += modifier.magazine_capacity(self);
        }
        (self.magazine_size as f32 * (1.0 + magazine_capacity)) as usize
    }

    fn multishot(&self) -> f32 {
        let mut multishot = 0.0;
        for modifier in self.modifier_list.iter() {
            multishot += modifier.multishot(self);
        }
        self.multishot * (1.0 + multishot)
    }

    fn reload_speed(&self) -> f32 {
        let mut reload_speed = 0.0;
        for modifier in self.modifier_list.iter() {
            reload_speed += modifier.reload_speed(self);
        }
        self.reload_time / (1.0 + reload_speed)
    }

    fn reload_delay(&self) -> f32 {
        self.reload_delay
    }

    fn status_list(&self) -> Vec<Status> {
        let base_damage = self.base_damage();

        let mut impact = 0.0;
        let mut puncture = 0.0;
        let mut slash = 0.0;
        for status in self.status_list.iter() {
            match status {
                Status::Physical(physical) => match physical {
                    Physical::Impact(damage) => impact += damage,
                    Physical::Puncture(damage) => puncture += damage,
                    Physical::Slash(damage) => slash += damage,
                },
                _ => {}
            }
        }

        let mut status_list: Vec<Status> = Vec::new();
        for modifier in self.modifier_list.iter() {
            for status in modifier.status_list(self) {
                let mut status = status.clone();
                match status {
                    Status::Physical(physical) => match physical {
                        Physical::Impact(_) => status.set_damage(status.damage() * impact),
                        Physical::Puncture(_) => status.set_damage(status.damage() * puncture),
                        Physical::Slash(_) => status.set_damage(status.damage() * slash),
                    },
                    _ => status.set_damage(status.damage() * base_damage),
                }
                status_list.push(status);
            }
        }

        status_list.extend(self.status_list.iter());
        status_list.merge()
    }

    fn modifier_list(&self) -> &Vec<Arc<dyn Modifier>> {
        &self.modifier_list
    }
}
