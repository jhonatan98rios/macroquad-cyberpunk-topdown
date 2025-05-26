use macroquad::prelude::*;
use crate::player::Player;
use crate::state::GameState;


#[derive(Clone)]
pub struct ExperienceOrb {
    pub position: Vec2,
    pub value: f32,
}

pub struct ExperienceSystem {
    experience_orbs: Vec<ExperienceOrb>,
}

impl ExperienceSystem {
    pub fn new() -> Self {

        ExperienceSystem {
            experience_orbs: Vec::new(),
        }
    }

    pub fn spawn_experience_orb(&mut self, position: Vec2, value: f32) {
        let orb = ExperienceOrb {
            position,
            value,
        };
        self.experience_orbs.push(orb);
    }
    
    pub fn update(&mut self, player: &mut Player) -> Option<GameState> {
        self.check_experience_orbs_collisions(player);

        if player.current_experience >= player.experience_to_next_level {
            player.level_up();
            return Some(GameState::LevelUp);
        }

        return None
    }

    pub fn check_experience_orbs_collisions(&mut self, player: &mut Player) {

        let mut orbs_to_remove = Vec::new();

        for orb in &self.experience_orbs {
            if player.position().x < orb.position.x + 5.0 &&
                player.position().x + player.size > orb.position.x &&
                player.position().y < orb.position.y + 5.0 &&
                player.position().y + player.size > orb.position.y {
                    player.add_experience(orb.value);
                    orbs_to_remove.push(orb.position);
            }
        }

        self.experience_orbs.retain(|orb| !orbs_to_remove.contains(&orb.position));
    }

    pub fn draw(&self) {
        for orb in &self.experience_orbs {
            draw_rectangle(
                orb.position.x,
                orb.position.y,
                5.0,
                5.0,
                BLUE
            );
        }
    }
}