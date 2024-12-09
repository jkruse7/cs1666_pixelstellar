use bevy::prelude::*;

pub const W1_ENEMY_SPEED: f32 = 100.;
pub const W1_ACCEL_RATE_X: f32 = 5000.;
pub const W1_ACCEL_RATE_Y: f32 = 10800.;
pub const W1_ANIM_TIME: f32 = 0.2;
pub const W1_SPRITE_HEIGHT: u32 = 50;
pub const W1_SPRITE_WIDTH: u32 = 30;

pub const W2_ENEMY_SPEED: f32 = 2.;
pub const W2_ACCEL_RATE_X: f32 = 1.;
pub const W2_ACCEL_RATE_Y: f32 = 1.;
pub const W2_SPRITE_HEIGHT: u32 = 62;
pub const W2_SPRITE_WIDTH: u32 = 40;


pub const W3_ENEMY_SPEED: f32 = 2.;
pub const W3_ACCEL_RATE_X: f32 = 1.;
pub const W3_ACCEL_RATE_Y: f32 = 1.;
pub const W3_SPRITE_HEIGHT: u32 = 62;
pub const W3_SPRITE_WIDTH: u32 = 40;

pub const W4_ENEMY_SPEED: f32 = 100.;
pub const W4_ACCEL_RATE_X: f32 = 5000.;
pub const W4_ACCEL_RATE_Y: f32 = 10800.;
pub const W4_ANIM_TIME: f32 = 0.2;
pub const W4_SPRITE_HEIGHT: u32 = 30;
pub const W4_SPRITE_WIDTH: u32 = 28;

pub const W6_ENEMY_SPEED: f32 = 2.;
pub const W6_ACCEL_RATE_X: f32 = 1.;
pub const W6_ACCEL_RATE_Y: f32 = 1.5;
pub const W6_SPRITE_HEIGHT: u32 = 40;
pub const W6_SPRITE_WIDTH: u32 = 40;

pub const W8_ENEMY_SPEED: f32 = 100.;
pub const W8_ACCEL_RATE_X: f32 = 5000.;
pub const W8_ACCEL_RATE_Y: f32 = 10800.;
pub const W8_ANIM_TIME: f32 = 0.2;
pub const W8_SPRITE_HEIGHT: u32 = 224;
pub const W8_SPRITE_WIDTH: u32 = 224;


#[derive(Component, Resource)]
pub struct EnemyHealth {
    pub hp: f32,
}

impl EnemyHealth {
    pub fn new(hp: f32) -> Self {
        Self {
            hp: hp,
        }
    }
}
