use bevy::ecs::query;
use bevy::{prelude::*, window::PresentMode};
use crate::core::gameplay::player::Player;

#[derive(Component)]
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
                info!("Collision detected between {:?} and {:?}", self, hitbox);
                return true;
            }
        }
        false
    }
    


}