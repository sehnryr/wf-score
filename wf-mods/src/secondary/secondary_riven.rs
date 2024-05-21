use super::common::*;

#[derive(Clone, PartialEq)]
pub struct SecondaryRiven {
    damage: f32,
    critical_chance: f32,
    critical_multiplier: f32,
    fire_rate: f32,
    ammo_maximum: f32,
    magazine_capacity: f32,
    multishot: f32,
    reload_speed: f32,
    status_chance: f32,
    status_list: Vec<Status>,
}

impl SecondaryRiven {
    pub fn new(
        damage: f32,
        critical_chance: f32,
        critical_multiplier: f32,
        fire_rate: f32,
        ammo_maximum: f32,
        magazine_capacity: f32,
        multishot: f32,
        reload_speed: f32,
        status_chance: f32,
        status_list: Vec<Status>,
    ) -> Self {
        Self {
            damage,
            critical_chance,
            critical_multiplier,
            fire_rate,
            ammo_maximum,
            magazine_capacity,
            multishot,
            reload_speed,
            status_chance,
            status_list,
        }
    }
}

#[modifier]
impl Modifier for SecondaryRiven {
    fn damage(&self, _context: &dyn Weapon) -> f32 {
        self.damage
    }

    fn critical_chance(&self, _context: &dyn Weapon) -> f32 {
        self.critical_chance
    }

    fn critical_multiplier(&self, _context: &dyn Weapon) -> f32 {
        self.critical_multiplier
    }

    fn fire_rate(&self, _context: &dyn Weapon) -> f32 {
        self.fire_rate
    }

    fn ammo_maximum(&self, _context: &dyn Weapon) -> f32 {
        self.ammo_maximum
    }

    fn magazine_capacity(&self, _context: &dyn Weapon) -> f32 {
        self.magazine_capacity
    }

    fn multishot(&self, _context: &dyn Weapon) -> f32 {
        self.multishot
    }

    fn reload_speed(&self, _context: &dyn Weapon) -> f32 {
        self.reload_speed
    }

    fn status_chance(&self, _context: &dyn Weapon) -> f32 {
        self.status_chance
    }

    fn status_list(&self, _context: &dyn Weapon) -> Vec<Status> {
        self.status_list.clone()
    }
}

impl std::fmt::Debug for SecondaryRiven {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("SecondaryRiven");
        if self.damage != 0.0 {
            f.field("damage", &self.damage);
        }
        if self.critical_chance != 0.0 {
            f.field("critical_chance", &self.critical_chance);
        }
        if self.critical_multiplier != 0.0 {
            f.field("critical_multiplier", &self.critical_multiplier);
        }
        if self.fire_rate != 0.0 {
            f.field("fire_rate", &self.fire_rate);
        }
        if self.ammo_maximum != 0.0 {
            f.field("ammo_maximum", &self.ammo_maximum);
        }
        if self.magazine_capacity != 0.0 {
            f.field("magazine_capacity", &self.magazine_capacity);
        }
        if self.multishot != 0.0 {
            f.field("multishot", &self.multishot);
        }
        if self.reload_speed != 0.0 {
            f.field("reload_speed", &self.reload_speed);
        }
        if self.status_chance != 0.0 {
            f.field("status_chance", &self.status_chance);
        }
        if !self.status_list.is_empty() {
            f.field("status_list", &self.status_list);
        }
        f.finish()
    }
}
