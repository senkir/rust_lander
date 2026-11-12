use std::collections::HashMap;
use crate::damage::health::Health;
use bevy::prelude::Component;

/// maps damage stage sprite indices to percent damage
/// where 1.0 is full health
#[derive(Component)]
pub struct DamageStage {
    full: usize,
    three_quarters: usize,
    half: usize,
    quarter: usize,
    empty: usize,
    critical: usize,
    default: usize,
}

impl Default for DamageStage {
    fn default() -> Self {
        DamageStage {
            full: 0,
            three_quarters: 1,
            half: 2,
            quarter: 3,
            critical: 4,
            empty: 5,
            default: 0
        }
    }
}

impl DamageStage {

    fn stage(&self, health_amount: f32) -> usize {
        match health_amount {
            f if f > 0.75 => self.full, //healthy
            f if f > 0.5 => self.three_quarters,  //damaged
            f if f > 0.25 => self.half,  //damaged
            f if f > 0.15 => self.quarter,
            f if f > 0.1 => self.critical,
            _ => self.empty, //critical
        }
    }

    pub fn damage_stage(&self, health: &Health) -> usize {
        self.stage(health.fraction())
    }
}

#[cfg(test)]
mod Test {
    use super::*;


    #[test]
    fn test_damageStage_default_health() {
        let stage = DamageStage::default();
        let health = Health::new(100);
        assert_eq!(stage.damage_stage(&health), 0);
    }

    #[test]
    fn test_damageStage_default_fraction() {
        let stage = DamageStage::default();
        assert_eq!(stage.stage(1.0), 0);
    }

    #[test]
    fn test_damageStage_default_fraction_empty() {
        let stage = DamageStage::default();
        assert_eq!(stage.stage(0.0), 5);
    }

    #[test]
    fn test_damageStage_default_fraction_three_quarters() {
        let stage = DamageStage::default();
        assert_eq!(stage.stage(0.75), 1);
    }

    #[test]
    fn test_damageStage_default_fraction_half() {
        let stage = DamageStage::default();
        assert_eq!(stage.stage(0.5), 2);
    }

    #[test]
    fn test_damageStage_default_fraction_quarter() {
        let stage = DamageStage::default();
        assert_eq!(stage.stage(0.25), 3);
    }

    #[test]
    fn test_damageStage_default_fraction_critical() {
        let stage = DamageStage::default();
        assert_eq!(stage.stage(0.11), 4);
    }
}
