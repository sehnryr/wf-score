use super::common::*;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ConditionOverload {
    /// The % of the total damage that a status must have to count as a condition
    ///
    /// Default is 0.0
    pub threshold: f32,
}

impl ConditionOverload {
    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }
}

#[modifier]
impl Modifier for ConditionOverload {
    fn damage(&self, context: &dyn Weapon) -> f32 {
        let status_list = context.status_list();
        let total_damage = status_list.damage();

        let mut status_count = 0;
        for status in status_list {
            if status.damage() / total_damage > self.threshold {
                status_count += 1;
            }
        }

        0.8 * status_count as f32
    }
}
