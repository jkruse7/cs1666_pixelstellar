use bevy::prelude::*;
use std::time::Duration;


pub const BLASTER_OFFSET_X: f32 = -5.;
pub const BLASTER_OFFSET_Y: f32 = -15.;

pub const MAX_FLIGHT_SPEED: f32 = 250.;
pub const PLAYER_MAX_SPEED: f32 = 250.;

pub const ACCEL_RATE_X: f32 = 5000.;
pub const ACCEL_RATE_Y: f32 = 10800.;

pub const ANIM_TIME: f32 = 0.2;

pub const SPRITE_HEIGHT: u32 = 50;
pub const SPRITE_WIDTH: u32 = 30;

pub const SPLASH_THRESHOLD: f32 = 1.5; //ratio of new ratio of water particles to old ratio of water particles to trigger splash (should be greater than 1)

pub const PLAYER_DAMAGE_SOUND_FILE: &str = "damage_sound.wav";
pub const PLAYER_DAMAGE_SOUND_DURATION: f32 = 1.5;

#[derive(Resource)]
pub struct PlayerRatioWaterParticles { //Ratio of player's hitbox that is in water
    pub number: f32,
}
impl PlayerRatioWaterParticles {
    pub fn new() -> Self {
        Self {
            number: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct PlayerSoundTracker {
    pub last_played: Duration,
}
impl PlayerSoundTracker {
    pub fn new() -> Self {
        Self {
            last_played: Duration::new(0, 0),
        }
    }
}