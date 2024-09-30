use bevy::prelude::*;

// Gravitational acceleration: 1px/frame^2 @60Hz == 3600px/s^2
const GRAVITY: f32 = 3600.;
const TERMINAL_VELOCITY: f32 = 750.;

#[derive(Component)]
pub struct Gravity {
    current_G: f32,
}

impl Gravity {
    pub fn new() -> Self {
        Self {
            current_G: 0.,
        }
    }

    pub fn update_G(&mut self, curr_velocity: &f32, deltat: &f32) {
        //self.current_G = GRAVITY;
        self.current_G = f32::max(-TERMINAL_VELOCITY, curr_velocity - GRAVITY * deltat);
    }

    pub fn get_G(&mut self) -> f32 {
        self.current_G
    }
    pub fn reset_G(&mut self) {
        self.current_G = 0.;
    }
}