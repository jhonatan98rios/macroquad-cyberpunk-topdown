use macroquad::prelude::*;
use crate::{render::Renderable};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum EnemyStatus {
    Pending,
    Live,
    Dead,
}

#[derive(Clone)]
pub struct Enemy {
    pub position: Vec2,
    pub size: Vec2,
    pub status: EnemyStatus,
    pub last_movement: Vec2,
    pub max_health: f32,
    pub health: f32,
}


#[derive(Clone, Copy)]
pub struct EnemyInfo {
    pub position: Vec2,
    pub status: EnemyStatus,
}

#[derive(Clone, Copy)]
pub struct EnemyView {
    pub position: Vec2,
    pub size: Vec2,
    pub alive: bool,
}

pub struct EnemyRenderable<'a> {
    pub enemy: &'a Enemy,
    pub texture: &'a Texture2D,
    pub current_frame: usize,
    pub flip_x: bool,
}

impl<'a> Renderable for EnemyRenderable<'a> {

    fn position(&self) -> Vec2 {
        self.enemy.position
    }

    fn draw(&self) -> () {

        let texture = Some(self.texture);

        match texture {
            Some(texture) => {
                /* let frame_width = self.enemy.size.x;
                let frame_height = self.enemy.size.y;

                let params = DrawTextureParams {
                    dest_size: Some(self.enemy.size),
                    flip_x: self.flip_x,
                    source: Some(Rect {
                        x: self.current_frame as f32 * frame_width,
                        y: texture.height(),
                        w: frame_width,
                        h: -frame_height,
                    }),
                    ..Default::default()
                };
        
                draw_texture_ex(
                    texture,
                    self.enemy.position.x,
                    self.enemy.position.y,
                    WHITE,
                    params
                ); */

                draw_rectangle(
                    self.enemy.position.x,
                    self.enemy.position.y,
                    self.enemy.size.x,
                    self.enemy.size.y,
                    RED,
                );
            }

            None => {
                draw_rectangle(
                    self.enemy.position.x,
                    self.enemy.position.y,
                    self.enemy.size.x,
                    self.enemy.size.y,
                    RED,
                );
            }
        }
    }
}