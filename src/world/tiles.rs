use bevy::prelude::*;

use crate::engine::hitbox::Hitbox;

#[derive(Component)]
pub struct tiles {
    pub hitbox: Hitbox,
}

impl tiles {
    pub fn new(width: f32, height: f32, offset: Vec2) -> Self {
        Self {
            hitbox: Hitbox::new(width, height, offset),
        }
    }
}
