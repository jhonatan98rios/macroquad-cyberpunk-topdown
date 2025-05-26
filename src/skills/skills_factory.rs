use std::collections::HashMap;

use crate::skills::simple_projectile::SimpleProjectileManager;
use crate::skills::force_field::ForceFieldManager;

pub struct SkillsFactory {}

impl SkillsFactory {

    pub fn create_simple_projectile_manager() -> SimpleProjectileManager {
        
        return SimpleProjectileManager {
            damage: 1.0,
            speed: 300.0,
            radius: 5.0,
            life: 2.0,
            cooldown: 0.5,
            is_active: true,
            timer: 0.0,
            projectiles: HashMap::new(),
        };
    }

    pub fn create_force_field_manager() -> ForceFieldManager {
        return ForceFieldManager {
            projectiles: HashMap::new(),
            damage: 0.1,
            radius: 128.0,
            is_active: true,
            initialized: false,
        };
    }
}