use bevy::prelude::*;

use crate::core::engine::hitbox::Hitbox;

#[derive(Component)]
pub struct Tiles {
    pub hitbox: Hitbox,
}

impl Tiles {
    pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
        Self {
            hitbox: Hitbox::new(width, height, offset),
        }
    }
}
