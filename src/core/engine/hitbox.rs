use bevy::ecs::query;
use bevy::{prelude::*, window::PresentMode};
use crate::core::gameplay::player::Player;
use crate::core::gameplay::enemy::Enemy;
use crate::core::world::tiles::tiles;

#[derive(Component, Clone)]
#[derive(Debug)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub offset: Vec2, //bottom left corner
}

#[derive(Component)]
pub struct DoNotSearchCollide;

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
        let self_tr = self.offset + Vec2::new(self.width,self.height);
        let other_tr = other.offset + Vec2::new(other.width,other.height);
        self.offset.x < other_tr.x && self_tr.x > other.offset.x && self.offset.y < other_tr.y && self_tr.y > other.offset.y
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
}