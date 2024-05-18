use super::common::*;

#[derive(Clone, PartialEq)]
pub struct MeleeRiven {
    damage: f32,
    critical_chance: f32,
    critical_multiplier: f32,
    status_chance: f32,
    attack_speed: f32,
    status_list: Vec<Status>,
}

impl MeleeRiven {
    pub fn new(
        damage: f32,
        critical_chance: f32,
        critical_multiplier: f32,
        status_chance: f32,
        attack_speed: f32,
        status_list: Vec<Status>,
    ) -> Self {
        Self {
            damage,
            critical_chance,
            critical_multiplier,
            status_chance,
            attack_speed,
            status_list,
        }
    }
}

#[modifier]
impl Modifier for MeleeRiven {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        self.damage
    }

    fn critical_chance(&self, _context: &dyn Weapon) -> f32 {
        self.critical_chance
    }

    fn critical_multiplier(&self, _context: &dyn Weapon) -> f32 {
        self.critical_multiplier
    }

    fn status_chance(&self, _context: &dyn Weapon) -> f32 {
        self.status_chance
    }

    fn attack_speed(&self, _context: &dyn Weapon) -> f32 {
        self.attack_speed
    }

    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        self.status_list.clone()
    }
}

impl std::fmt::Debug for MeleeRiven {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("MeleeRiven");
        if self.damage != 0.0 {
            f.field("damage", &self.damage);
        }
        if self.critical_chance != 0.0 {
            f.field("critical_chance", &self.critical_chance);
        }
        if self.critical_multiplier != 0.0 {
            f.field("critical_multiplier", &self.critical_multiplier);
        }
        if self.status_chance != 0.0 {
            f.field("status_chance", &self.status_chance);
        }
        if self.attack_speed != 0.0 {
            f.field("attack_speed", &self.attack_speed);
        }
        if !self.status_list.is_empty() {
            f.field("status_list", &self.status_list);
        }
        f.finish()
    }
}
