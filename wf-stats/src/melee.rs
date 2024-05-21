use std::sync::Arc;

use derivative::Derivative;

use crate::modifier::{Modifier, WeaponModifiers};
use crate::status::{Physical, Status, StatusesImpl};
use crate::weapon::Weapon;

#[derive(Derivative, Default, Clone)]
#[derivative(Debug)]
pub struct Melee {
    critical_chance: f32,
    critical_multiplier: f32,
    status_chance: f32,
    attack_speed: f32,
    status_list: Vec<Status>,
    #[derivative(Debug = "ignore")]
    modifier_list: Vec<Arc<dyn Modifier>>,
}

impl Melee {
    pub fn new(
        critical_chance: f32,
        critical_multiplier: f32,
        status_chance: f32,
        attack_speed: f32,
        status_list: Vec<Status>,
    ) -> Self {
        Self {
            critical_chance,
            critical_multiplier,
            status_chance,
            attack_speed,
            status_list,
            modifier_list: Vec::new(),
        }
    }

    fn base_damage(&self) -> f32 {
        self.status_list.iter().map(|status| status.damage()).sum()
    }
}

impl WeaponModifiers for Melee {
    fn add_modifier(&mut self, modifier: Arc<dyn Modifier>) {
        self.modifier_list.push(modifier);
    }
}

impl Weapon for Melee
where
    Self: Default,
{
    fn fire_rate(&self) -> f32 {
        Default::default()
    }

    fn ammo_maximum(&self) -> usize {
        Default::default()
    }

    fn magazine_capacity(&self) -> usize {
        Default::default()
    }

    fn multishot(&self) -> f32 {
        Default::default()
    }

    fn reload_speed(&self) -> f32 {
        Default::default()
    }

    fn reload_delay(&self) -> f32 {
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

    fn attack_speed(&self) -> f32 {
        let mut attack_speed = 0.0;
        for modifier in self.modifier_list.iter() {
            attack_speed += modifier.attack_speed(self);
        }
        self.attack_speed * (1.0 + attack_speed)
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
