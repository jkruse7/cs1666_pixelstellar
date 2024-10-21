use std::convert;

use bevy::{
    prelude::*,
    window::PresentMode,
    ecs::query,
};

use crate::particle::components::ParticleData;
use crate::particle::systems::*;

use crate::ParticleMap;
use crate::{
    gameplay::{
        player::Player,
        enemy::Enemy,
    },
    world::tiles::tiles,
};



#[derive(Component)]
pub struct DoNotSearchCollide;

#[derive(Component, Clone, Debug)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub offset: Vec2, //center of the hitbox
}

impl Hitbox {
    pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
        Self {
            width,
            height,
            offset,
        }
    }
    pub fn collides_with(&self, other: &Hitbox) -> bool {
        //tr = topright corner
        let self_tr = self.offset + Vec2::new(self.width,self.height)/2.0;
        let self_bl = self.offset - Vec2::new(self.width,self.height)/2.0;
        let other_bl = other.offset - Vec2::new(other.width,other.height)/2.0;
        let other_tr = other.offset + Vec2::new(other.width,other.height)/2.0;
        self_tr.x > other_bl.x && self_bl.x < other_tr.x && self_tr.y > other_bl.y && self_bl.y < other_tr.y
    }
    pub fn all_player_collisions(&self, hitboxes: &Query<&Hitbox, Without<Player>>)  -> bool {
        for hitbox in hitboxes.iter() {
            if self.collides_with(hitbox) {
                //info!("Collision detected between {:?} and {:?}", self, hitbox);
                return true;
            }
        }
        false
    }
    pub fn player_enemy_collision(&self, hitboxes: &Query<&Hitbox, (With<Enemy>, Without<Player>)>)  -> bool {
        for hitbox in hitboxes.iter() {
            if self.collides_with(hitbox) {
                return true;
            }
        }
        false
    }
    pub fn all_enemy_collisions(&self, hitboxes: &Query<&Hitbox, Without<Enemy>>)  -> bool {
        for hitbox in hitboxes.iter() {
            if self.collides_with(hitbox) {
                //info!("Enemy Collision detected between {:?} and {:?}", self, hitbox);
                return true;
            }
        }
        false
    }
    pub fn tile_collision(&self, tile: &tiles) -> bool {
        self.collides_with(&tile.hitbox)
    }

    pub fn all_tile_collisions(&self, tiles: &Query<&tiles>) -> bool {
        for tile in tiles.iter() {
            if self.tile_collision(tile) {
                return true;
            }
        }
        false
    }
    pub fn contains(&self, position: &Vec2) -> bool {
        // 假设 hitbox 以中心为原点
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        position.x >= self.offset.x - half_width &&
        position.x <= self.offset.x + half_width &&
        position.y >= self.offset.y - half_height &&
        position.y <= self.offset.y + half_height
    }

    pub fn are_all_grid_tiles_air(&self, mut map: ResMut<ParticleMap>) -> bool {
        let (top_left_x, top_left_y, bottom_right_x, bottom_right_y) = self.get_grid_tiles_to_check();
        for x in top_left_x..=bottom_right_x {
            for y in bottom_right_y..=top_left_y {
                if map.get(x, y) != ParticleData::Air {
                    return false;
                }
            }   
        }
        true
    }

    // return the grid position of the top left and bottom right corners of the hitbox
    // (top_left_x, top_left_y, bottom_right_x, bottom_right_y) 
    pub fn get_grid_tiles_to_check(&self) -> (i32, i32, i32, i32) { //
        let top_left_grid_pos = convert_to_grid_position(self.offset.x - self.width / 2.0, self.offset.y + self.height / 2.0);
        let bottom_right_grid_pos = convert_to_grid_position(self.offset.x + self.width / 2.0, self.offset.y - self.height / 2.0);
        (top_left_grid_pos.0, top_left_grid_pos.1, bottom_right_grid_pos.0, bottom_right_grid_pos.1)
    }
}