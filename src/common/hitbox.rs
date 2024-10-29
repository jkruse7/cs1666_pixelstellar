use bevy::prelude::*;

use crate::entities::particle::components::ParticleData;
use crate::entities::particle::systems::*;

use crate::{
    entities::particle::resources::ParticleMap,
    entities::player::components::Player,
    entities::enemy::components::Enemy,
};

const STEP_HEIGHT: f32 = 5.;


//#[derive(Component)]
//pub struct DoNotSearchCollide;

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
    pub fn on_top_of(&self, other: &Hitbox) -> bool{
        // im within x
        if (other.offset.x - (self.width/2.) <= self.offset.x + (self.width/2.)
            && other.offset.x + (self.width/2.) >= self.offset.x - (self.width/2.)){
            // if top of particle is less than or equal to bottom of player
            if (other.offset.y + (other.height / 2.) <= self.offset.y - (self.height/2.)){
                return true;
            }
        }
        return false;
    }
    pub fn on_top_of_all(&self, hitboxes: &Query<&Hitbox, (Without<Enemy>, Without<Player>)>) -> bool{
        for hitbox in hitboxes.iter() {
            if self.on_top_of(hitbox) {
                return true;
            }
        }
        false
    }
    pub fn player_step(&self, hitboxes: &Query<&Hitbox,(Without<Enemy>, Without<Player>)>)  -> f32 {
        for hitbox in hitboxes.iter() {
            // if you collide with a particle
            if self.collides_with(hitbox){
                // ... and the top of the particle hitbox is less than 
                let other_top_y = hitbox.offset.y + (hitbox.height / 2.);
                let self_bottom_y = self.offset.y - (self.height / 2.);
                info!("Other: {}, me: {}",
                    other_top_y, self_bottom_y);
                let offset = other_top_y - self_bottom_y;
                if offset < STEP_HEIGHT{
                    return offset
                } else {
                    return -1.
                }
            }
        }
        0.
    }
    pub fn all_player_collisions(&self, hitboxes: &Query<&Hitbox, (Without<Enemy>, Without<Player>)>)  -> bool {
        for hitbox in hitboxes.iter() {
            if self.collides_with(hitbox) {
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
    /*pub fn contains(&self, position: &Vec2) -> bool {
        // 假设 hitbox 以中心为原点
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        position.x >= self.offset.x - half_width &&
        position.x <= self.offset.x + half_width &&
        position.y >= self.offset.y - half_height &&
        position.y <= self.offset.y + half_height
    }

    pub fn are_all_grid_tiles_air(&self, map: &ResMut<ParticleMap>) -> bool {
        let (top_left_x, top_left_y, bottom_right_x, bottom_right_y) = self.get_grid_tiles_to_check();
        for x in top_left_x..=bottom_right_x {
            for y in bottom_right_y..=top_left_y {
                if map.get(x, y) != ParticleData::Air {
                    return false;
                }
            }   
        }
        true
    }*/

    pub fn are_any_grid_tiles_water(&self, map: &ResMut<ParticleMap>) -> bool {
        let (top_left_x, top_left_y, bottom_right_x, bottom_right_y) = self.get_grid_tiles_to_check();
        for x in top_left_x..=bottom_right_x {
            for y in bottom_right_y..=top_left_y {
                if map.get(x, y) == ParticleData::Water {
                    return true;
                }
            }   
        }
        false
    }

    pub fn ratio_of_water_grid_tiles(&self, map: &ResMut<ParticleMap>) -> f32 {
        let (top_left_x, top_left_y, bottom_right_x, bottom_right_y) = self.get_grid_tiles_to_check();
        let mut count = 0;
        let mut no_count = 0;
        for x in top_left_x..=bottom_right_x {
            for y in bottom_right_y..=top_left_y {
                if map.get(x, y) == ParticleData::Water {
                    count+=1;
                }
                else {
                    no_count+=1;
                }
            }   
        }
        count as f32 / (count + no_count) as f32
    }
    // return the grid position of the top left and bottom right corners of the hitbox
    // (top_left_x, top_left_y, bottom_right_x, bottom_right_y) 
    pub fn get_grid_tiles_to_check(&self) -> (i32, i32, i32, i32) { //
        let top_left_grid_pos = convert_to_grid_position(self.offset.x - self.width / 2.0, self.offset.y + self.height / 2.0);
        let bottom_right_grid_pos = convert_to_grid_position(self.offset.x + self.width / 2.0, self.offset.y - self.height / 2.0);
        (top_left_grid_pos.0, top_left_grid_pos.1, bottom_right_grid_pos.0, bottom_right_grid_pos.1)
    }
}